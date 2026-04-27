# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is a **Rust workspace**, **specs** + JSON Schema, **`themes/`** packs (each with **`theme.jsonc`**), and an optional **Nix devShell**. **Bootstrap** is a **standalone CLI**. Each **`theme.jsonc`** carries **`tokens`** (Base16 palette), **`fonts`**, optional **`assets`**, and optional **`targets`** (mappings + optional **`apply_quick`** / **`apply_nix`** blocks per target). **Builtin adapters** interpret that file and apply **`apply-quick`** (live paths) or **`apply-nix`** (Nix tree only). First-party HM/NixOS modules are **deferred**—see `nix/modules/README.md`.

## Pillars

1. **Fast iteration — `chromamancer apply-quick`** writes **straight to live target paths** using `targets.<id>.apply_quick` when present; **`apply-nix`** writes into the **Nix config tree** using `apply_nix`; **`switch`** materializes (see `specs/SPEC.md`).
2. **Nix (optional)** — `nix/flake.nix` is **devShell** for now; users own integration. When Nix installs the same paths, **rebuild overwrites** CLI-written files there.
3. **Schematic themes** — `specs/schemas/` validates **`theme.jsonc`**; `specs/SPEC.md` is the human index.
4. **Theme packs** — directories under `themes/<id>/` with **`theme.jsonc`** (+ `assets/`).

## Directory structure

```
.
├── Cargo.toml              # workspace root
├── crates/cli/             # binary chromamancer; builtin adapters (e.g. src/targets/)
├── nix/
│   ├── flake.nix
│   └── modules/            # deferred HM/NixOS
├── specs/
│   ├── SPEC.md
│   └── schemas/
└── themes/
    ├── README.md
    └── _template/
```

## Data flow

```
specs/schemas  ──validate──►  themes/*/theme.jsonc
                                      │
              ┌───────────────────────┴────────────────────────┐
              ▼                                                ▼
    builtin adapters (Rust)                         user Nix / HM (optional)
    theme + targets → per-target artifacts          installs generated files on switch
```

## Technology stack

- **Rust** — CLI and adapters.
- **Nix** — dev shell; future packaging/modules.
- **JSON Schema** — `specs/schemas/scheme-v1.schema.json`; **adapter behavior** is code + tests.

## Key decisions

- **Theme file** — **`themes/<id>/theme.jsonc`** (JSONC); **`targets.<id>`** may include **`mappings`**, **`apply_quick`**, **`apply_nix`**, **`logic`**, **`overrides`** (optional pieces per target).
- **Workspace layout** — `crates/cli`; binary **`chromamancer`**.
- **Spec before codegen** — version schemas before locking generators.
- **v1 palette** — Base16 `base00`–`base0F`, **`#RRGGBBAA`**, required **`fonts.ui` / `fonts.mono`** (families only).
- **Bootstrap delivery** — standalone CLI; no required flake/HM from this repo.
- **Target adapters** — Rust builtins: defaults, mode routing, imperative **`logic`** hooks referenced from `theme.jsonc`.
- **Apply model** — **apply-quick** vs **apply-nix**; Nix **switch** authoritative for paths it owns.

## Future considerations

- Watch mode / reload helpers.
- Tighter JSON Schemas per `targets.<id>`.
- CI: `cargo test`, validate `themes/*`, flake checks.
