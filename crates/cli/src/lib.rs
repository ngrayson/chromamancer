//! Theme resolution (v3) and target emit helpers.

mod hyprland;
mod jsonc;
mod kitty;
mod theme_v3;

pub use hyprland::{render_hyprland_decorations, HyprlandEmitError};
pub use kitty::{render_kitty_colors, KittyEmitError};
pub use theme_v3::{resolve_theme_v3, ResolvedThemeV3, ThemeV3Error};

use std::path::Path;

/// Load `targets/<target_id>/mapping.jsonc` relative to the chromamancer project root
/// (`CHROMAMANCER_TARGETS_DIR` or an ancestor of `theme_path` / cwd containing `targets/`).
pub fn resolve_project_root(
    theme_path: &Path,
    targets_dir_override: Option<&Path>,
    target_id: &str,
) -> Result<std::path::PathBuf, String> {
    if let Some(root) = targets_dir_override {
        let p = root.join("targets").join(target_id).join("mapping.jsonc");
        if p.is_file() {
            return Ok(root.to_path_buf());
        }
        return Err(format!(
            "no file at {} (expected project root with targets/{target_id}/mapping.jsonc)",
            p.display()
        ));
    }
    if let Ok(env_root) = std::env::var("CHROMAMANCER_TARGETS_DIR") {
        let root = std::path::PathBuf::from(env_root);
        let p = root.join("targets").join(target_id).join("mapping.jsonc");
        if p.is_file() {
            return Ok(root);
        }
        return Err(format!(
            "CHROMAMANCER_TARGETS_DIR={} does not contain targets/{target_id}/mapping.jsonc",
            root.display()
        ));
    }
    if let Some(root) = find_targets_root(theme_path, target_id) {
        return Ok(root);
    }
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    if let Some(root) = find_targets_root(&cwd, target_id) {
        return Ok(root);
    }
    Err(format!(
        "could not find targets/{target_id}/mapping.jsonc: set CHROMAMANCER_TARGETS_DIR \
         to the chromamancer repo root, or run from that tree"
    ))
}

fn find_targets_root(start: &Path, target_id: &str) -> Option<std::path::PathBuf> {
    let file = std::path::Path::new("targets")
        .join(target_id)
        .join("mapping.jsonc");
    for anc in start.ancestors() {
        let p = anc.join(&file);
        if p.is_file() {
            return Some(anc.to_path_buf());
        }
    }
    None
}

pub fn load_target_mapping_shims(
    project_root: &Path,
    target_id: &str,
) -> Result<std::collections::HashMap<String, String>, String> {
    let path = project_root
        .join("targets")
        .join(target_id)
        .join("mapping.jsonc");
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let v: serde_json::Value = jsonc::parse_jsonc(&raw).map_err(|e| {
        format!(
            "JSON in {}: {e}",
            path.display()
        )
    })?;
    let obj = v
        .get("shim_to_native")
        .and_then(|x| x.as_object())
        .ok_or_else(|| format!("{}: missing shim_to_native object", path.display()))?;
    let mut map = std::collections::HashMap::new();
    for (shim, native_val) in obj {
        match native_val {
            serde_json::Value::String(s) => {
                map.insert(shim.clone(), s.clone());
            }
            serde_json::Value::Array(_) => {
                return Err(format!(
                    "{}: shim_to_native.{shim} must be a string for target {target_id} (arrays reserved for e.g. Albert)",
                    path.display()
                ));
            }
            _ => {
                return Err(format!(
                    "{}: shim_to_native.{shim} must be a string",
                    path.display()
                ));
            }
        }
    }
    Ok(map)
}
