use std::io::stdout;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Parser)]
#[command(name = "chromamancer", version, about = "Spec-driven desktop theming")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Experimental full-screen UI
    Tui,
    /// Write colors to live config paths (see specs/SPEC.md)
    #[command(name = "apply-quick", visible_alias = "apply_quick")]
    ApplyQuick {
        #[command(subcommand)]
        target: ApplyQuickTarget,
    },
}

#[derive(Subcommand)]
enum ApplyQuickTarget {
    /// Emit kitty color directives (include from kitty.conf)
    Kitty(KittyArgs),
    /// Emit Hyprland border / decoration color snippet (source from hyprland.conf)
    Hyprland(HyprlandArgs),
}

#[derive(clap::Args)]
struct KittyArgs {
    /// Path to theme.jsonc (metadata.schema_version must be "3")
    #[arg(short = 't', long)]
    theme: PathBuf,
    /// Output file (default: ~/.config/kitty/chromamancer-colors.conf)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Directory that contains targets/ (default: CHROMAMANCER_TARGETS_DIR or auto-discover)
    #[arg(long)]
    targets_dir: Option<PathBuf>,
    /// Print generated config to stdout instead of writing a file
    #[arg(long)]
    stdout: bool,
}

#[derive(clap::Args)]
struct HyprlandArgs {
    /// Path to theme.jsonc (metadata.schema_version must be "3")
    #[arg(short = 't', long)]
    theme: PathBuf,
    /// Output file (default: ~/.config/hypr/chromamancer-decorations.conf)
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(long)]
    targets_dir: Option<PathBuf>,
    #[arg(long)]
    stdout: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Tui => run_tui(),
        Commands::ApplyQuick { target } => match target {
            ApplyQuickTarget::Kitty(args) => apply_kitty_quick(args),
            ApplyQuickTarget::Hyprland(args) => apply_hyprland_quick(args),
        },
    }
}

fn apply_kitty_quick(args: KittyArgs) -> Result<()> {
    let resolved = chromamancer::resolve_theme_v3(&args.theme)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let root = chromamancer::resolve_project_root(
        &args.theme,
        args.targets_dir.as_deref(),
        "kitty",
    )
    .map_err(anyhow::Error::msg)?;
    let map =
        chromamancer::load_target_mapping_shims(&root, "kitty").map_err(anyhow::Error::msg)?;
    let snippet = chromamancer::render_kitty_colors(&resolved, &map)
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    if args.stdout {
        print!("{snippet}");
        return Ok(());
    }

    let out = args.output.unwrap_or_else(default_kitty_out);
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create directory {}", parent.display()))?;
    }
    std::fs::write(&out, &snippet).with_context(|| format!("write {}", out.display()))?;
    eprintln!("Wrote {}", out.display());
    Ok(())
}

fn default_kitty_out() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/kitty/chromamancer-colors.conf")
}

fn apply_hyprland_quick(args: HyprlandArgs) -> Result<()> {
    let resolved = chromamancer::resolve_theme_v3(&args.theme)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let root = chromamancer::resolve_project_root(
        &args.theme,
        args.targets_dir.as_deref(),
        "hyprland",
    )
    .map_err(anyhow::Error::msg)?;
    let map =
        chromamancer::load_target_mapping_shims(&root, "hyprland").map_err(anyhow::Error::msg)?;
    let snippet = chromamancer::render_hyprland_decorations(&resolved, &map)
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    if args.stdout {
        print!("{snippet}");
        return Ok(());
    }

    let out = args.output.unwrap_or_else(default_hyprland_out);
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create directory {}", parent.display()))?;
    }
    std::fs::write(&out, &snippet).with_context(|| format!("write {}", out.display()))?;
    eprintln!("Wrote {}", out.display());
    Ok(())
}

fn default_hyprland_out() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/hypr/chromamancer-decorations.conf")
}

fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("chromamancer")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
