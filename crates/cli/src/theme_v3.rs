use std::collections::HashMap;

use serde::Deserialize;

use crate::jsonc;

#[derive(Debug, Deserialize)]
struct ThemeFile {
    metadata: Metadata,
    tokens: HashMap<String, String>,
    shim_assign: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    #[allow(dead_code)]
    name: String,
    schema_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedColor {
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Transparent,
}

#[derive(Debug, Clone)]
pub struct ResolvedThemeV3 {
    pub name: String,
    pub colors: HashMap<String, ResolvedColor>,
}

#[derive(Debug, thiserror::Error)]
pub enum ThemeV3Error {
    #[error("expected theme metadata.schema_version \"3\", got \"{0}\"")]
    NotV3(String),
    #[error("unknown token `{token}` referenced by shim `{shim}`")]
    UnknownToken { shim: String, token: String },
    #[error("invalid color literal for shim `{shim}`: {reason}")]
    BadLiteral { shim: String, reason: String },
    #[error("transparent is not allowed for shim `{0}` in this target")]
    TransparentNotAllowed(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn resolve_theme_v3(path: &std::path::Path) -> Result<ResolvedThemeV3, ThemeV3Error> {
    let raw = std::fs::read_to_string(path)?;
    resolve_theme_v3_from_str(&raw)
}

pub fn resolve_theme_v3_from_str(raw: &str) -> Result<ResolvedThemeV3, ThemeV3Error> {
    let t: ThemeFile = serde_json::from_value(jsonc::parse_jsonc(raw)?)?;
    if t.metadata.schema_version != "3" {
        return Err(ThemeV3Error::NotV3(t.metadata.schema_version));
    }
    let mut colors = HashMap::with_capacity(t.shim_assign.len());
    for (shim, raw_val) in &t.shim_assign {
        let c = resolve_shim_value(shim, raw_val, &t.tokens)?;
        colors.insert(shim.clone(), c);
    }
    Ok(ResolvedThemeV3 {
        name: t.metadata.name,
        colors,
    })
}

fn resolve_shim_value(
    shim: &str,
    raw: &str,
    tokens: &HashMap<String, String>,
) -> Result<ResolvedColor, ThemeV3Error> {
    let s = raw.trim();
    if s == "transparent" {
        return Ok(ResolvedColor::Transparent);
    }
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex_shim(shim, hex);
    }
    let token_val = tokens.get(s).ok_or_else(|| ThemeV3Error::UnknownToken {
        shim: shim.into(),
        token: s.into(),
    })?;
    let token_val = token_val.trim();
    if token_val == "transparent" {
        return Ok(ResolvedColor::Transparent);
    }
    if let Some(hex) = token_val.strip_prefix('#') {
        return parse_hex_shim(shim, hex);
    }
    Err(ThemeV3Error::BadLiteral {
        shim: shim.into(),
        reason: format!("token {s} must be #RRGGBB or #RRGGBBAA"),
    })
}

fn parse_hex_shim(shim: &str, hex: &str) -> Result<ResolvedColor, ThemeV3Error> {
    let hex = hex.trim();
    let (r, g, b, a) = match hex.len() {
        6 => {
            let v = u32::from_str_radix(hex, 16).map_err(|_| ThemeV3Error::BadLiteral {
                shim: shim.into(),
                reason: "invalid hex".into(),
            })?;
            (
                ((v >> 16) & 0xff) as u8,
                ((v >> 8) & 0xff) as u8,
                (v & 0xff) as u8,
                255u8,
            )
        }
        8 => {
            let v = u64::from_str_radix(hex, 16).map_err(|_| ThemeV3Error::BadLiteral {
                shim: shim.into(),
                reason: "invalid hex".into(),
            })?;
            (
                ((v >> 24) & 0xff) as u8,
                ((v >> 16) & 0xff) as u8,
                ((v >> 8) & 0xff) as u8,
                (v & 0xff) as u8,
            )
        }
        _ => {
            return Err(ThemeV3Error::BadLiteral {
                shim: shim.into(),
                reason: format!("expected 6 or 8 hex digits after #, got length {}", hex.len()),
            });
        }
    };
    Ok(ResolvedColor::Rgba { r, g, b, a })
}

impl ResolvedColor {
    /// Kitty accepts `#rrggbb` or `#rrggbbaa`.
    pub fn to_kitty_hex(&self) -> Option<String> {
        match self {
            ResolvedColor::Transparent => None,
            ResolvedColor::Rgba { r, g, b, a } => {
                if *a == 255 {
                    Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
                } else {
                    Some(format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a))
                }
            }
        }
    }
}
