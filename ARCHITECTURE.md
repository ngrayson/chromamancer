# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is organized as a **Rust workspace** plus **Nix**, **machine-readable specs**, and **scheme packs**. **Scheme data** (`scheme.json`) holds the portable look (Base16 colors, font families, assets). **Target adapters** in code map that data onto each app and hold any non-trivial logic; optional **`target_overrides`** in the scheme layer exceptions on top. Apply **interactively** (CLI) or **declaratively** (Nix / Home Manager) using the same scheme + adapters.

## Pillars

1. **Fast iteration** — `crates/cli/` binary `chromamancer` (stub today) will apply schemes to running configs.
2. **Declarative Nix** — `nix/flake.nix` provides a dev shell; `modules/` will host HM/NixOS entrypoints that read the same scheme artifacts as the CLI.
3. **Schematic color schemes** — `specs/schemas/` defines validated scheme files; `specs/SPEC.md` is the human index.
4. **Scheme creation** — authors add directories under `schemes/` conforming to the active schema.

## Directory structure

```
.
├── Cargo.toml              # workspace root
├── crates/cli/             # CLI binary `chromamancer`; target adapters live here (e.g. `src/targets/`)
├── nix/
│   ├── flake.nix
│   └── modules/            # Home Manager / NixOS stubs
├── specs/
│   ├── SPEC.md
│   └── schemas/            # JSON Schema (etc.)
└── schemes/
    ├── README.md
    └── _template/          # copy to start a new scheme
```

## Data flow

```
specs/schemas  ──validate──►  schemes/*/scheme.json
                                      │
              ┌───────────────────────┴────────────────────────┐
              ▼                                                ▼
    target adapters (Rust)                           Nix / HM modules
    Base16+fonts+assets → per-app configs            same scheme + adapters
```

## Technology stack

- **Rust** — CLI (`ratatui` placeholder UI, same baseline as Stellarium cli-tool template).
- **Nix** — `nix develop` / future modules.
- **JSON Schema** — draft 2020-12 for `specs/schemas/scheme-v1.schema.json` (scheme data); **target adapters** are code + tests, not the schema file.

## Key decisions

- **Workspace layout** — generic path `crates/cli` keeps the Stellarium template independent of project name; the binary name remains `chromamancer`.
- **Spec before codegen** — Nix and Rust generators should target **versioned** schema files to avoid drift.
- **v1 palette** — canonical keys are **Base16** (`base00`–`base0F`) with **`#RRGGBBAA`**, plus required **global fonts** (`fonts.ui`, `fonts.mono`).
- **v1 scheme file** — `schemes/<id>/scheme.json` is **JSONC** on disk (comments allowed); validation runs on the parsed JSON value. Nix `fromJSON` needs strict JSON—use a build-time export or parser (see `specs/SPEC.md`).
- **Target adapters** — default Base16→app mapping and transforms live in **chromamancer** (incremental rollout per target); optional **`target_overrides`** in scheme for exceptions.

## Future considerations

- Watch mode / IPC for Quickshell and Hyprland reload.
- Optional TOML or other formats if we add a single canonical parser step and stable JSON projection.
- CI: `cargo test`, schema validation for `schemes/*`, and Nix flake checks.
