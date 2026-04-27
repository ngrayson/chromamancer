# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is organized as a **Rust workspace** plus **Nix**, **machine-readable specs**, and **scheme packs**. The long-term goal: one canonical description of a look (colors + assets + target bindings), applied either **interactively** (CLI) or **declaratively** (Nix / Home Manager).

## Pillars

1. **Fast iteration** — `crates/cli/` binary `chromamancer` (stub today) will apply schemes to running configs.
2. **Declarative Nix** — `nix/flake.nix` provides a dev shell; `modules/` will host HM/NixOS entrypoints that read the same scheme artifacts as the CLI.
3. **Schematic color schemes** — `specs/schemas/` defines validated scheme files; `specs/SPEC.md` is the human index.
4. **Scheme creation** — authors add directories under `schemes/` conforming to the active schema.

## Directory structure

```
.
├── Cargo.toml              # workspace root
├── crates/cli/             # CLI crate (package name / binary: chromamancer)
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
specs/schemas  ──validate──►  schemes/*     ──read──►  CLI apply
                                   │                    │
                                   └────────────────────┴──►  Nix modules / HM
```

## Technology stack

- **Rust** — CLI (`ratatui` placeholder UI, same baseline as Stellarium cli-tool template).
- **Nix** — `nix develop` / future modules.
- **JSON Schema** — scheme contract draft in `specs/schemas/scheme-v1.schema.json`.

## Key decisions

- **Workspace layout** — generic path `crates/cli` keeps the Stellarium template independent of project name; the binary name remains `chromamancer`.
- **Spec before codegen** — Nix and Rust generators should target **versioned** schema files to avoid drift.

## Future considerations

- Watch mode / IPC for Quickshell and Hyprland reload.
- Optional TOML scheme format alongside JSON if ergonomics demand it (single schema mapping).
- CI: `cargo test`, schema validation for `schemes/*`, and Nix flake checks.
