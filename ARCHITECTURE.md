# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is organized as a **Rust workspace** plus **Nix**, **machine-readable specs**, and **scheme packs**. **Scheme data** (`scheme.json`) holds the portable look (Base16 colors, font families, assets). **Target adapters** in code map that data onto each app and hold any non-trivial logic; optional **`target_overrides`** in the scheme layer exceptions on top. **CLI** supports **fast-iterative** targets during development; **NixOS / Home Manager** is the **system of record** and overwrites **all** generated outputs on rebuild—including fast targets (see `specs/SPEC.md`, **Apply model**).

## Pillars

1. **Fast iteration** — CLI applies **fast-iterative** targets (fragment regen + reload where supported); ephemeral until Nix captures the same inputs.
2. **Declarative Nix** — `nix/flake.nix` provides a dev shell; `modules/` host HM/NixOS entrypoints. **Rebuild overwrites every chromamancer output**, including those that also support CLI apply.
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
- **Apply model** — **fast-iterative** vs **rebuild-only** targets (per adapter); **`nixos-rebuild` / `home-manager switch` is authoritative** and overwrites CLI-written files for all targets. See `specs/SPEC.md`.

## Future considerations

- Watch mode / IPC for Quickshell and Hyprland reload.
- Optional TOML or other formats if we add a single canonical parser step and stable JSON projection.
- CI: `cargo test`, schema validation for `schemes/*`, and Nix flake checks.
