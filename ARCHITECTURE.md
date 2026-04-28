# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is a **Rust workspace**, **specs** + JSON Schema, **`themes/`** packs (**`theme.jsonc`**), repo-level **`targets/<id>/mapping.jsonc`**, and an optional **Nix devShell**. **Bootstrap** is a **standalone CLI**.

**Theme v3 (preferred):** arbitrary **`tokens`** and **`shim_assign`** → shared semantic shims ([`specs/shim-colors.md`](specs/shim-colors.md)). Target files use **`shim_to_native`** ([`target-mapping-v2.schema.json`](specs/schemas/target-mapping-v2.schema.json)).

**Theme v2:** **`tokens`** + **`canonical_assign`** ([`specs/canonical-colors.md`](specs/canonical-colors.md)); mappings use **`canonical_to_native`**.

**Theme v1 (legacy):** fixed Base16 `base00`–`base0F` + optional **`targets.<id>`** (`mappings`, `apply_quick`, `apply_nix`).

**Target mappings:** **`targets/<id>/mapping.jsonc`** — **v2 file** (`metadata.schema_version` **`"2"`**, **`shim_to_native`**) pairs with **theme v3**; **v1 file** pairs with **theme v2**. Optional **`apply_nix`** defaults (see [`targets/README.md`](targets/README.md)). Discovered via **`CHROMAMANCER_TARGETS_DIR`** (repo root by default).

**Builtin adapters** resolve **theme → resolved color table (shims or canonical) → native artifacts** for **`apply-quick`** / **`apply-nix`**. First-party HM/NixOS modules are **deferred**—`nix/modules/README.md`.

## Pillars

1. **Fast iteration — `chromamancer apply-quick`** writes live paths; **`apply-nix`** writes under your Nix tree; **`switch`** materializes (`specs/SPEC.md`).
2. **Nix (optional)** — `nix/flake.nix` is **devShell**; rebuild overwrites overlapping quick output.
3. **Schematic contracts** — `specs/schemas/` (`theme-v1`, **`theme-v2`**, **`theme-v3`**, **`target-mapping-v1`**, **`target-mapping-v2`**); [`SPEC.md`](specs/SPEC.md) + [`shim-colors.md`](specs/shim-colors.md) + [`canonical-colors.md`](specs/canonical-colors.md).
4. **Theme packs** — `themes/<id>/theme.jsonc` + optional **`assets/`**.

## Directory structure

```
.
├── Cargo.toml
├── crates/cli/
├── nix/
├── specs/
│   ├── SPEC.md
│   ├── canonical-colors.md
│   ├── logic-registry.md
│   └── schemas/
│       ├── theme-v1.schema.json
│       ├── theme-v2.schema.json
│       ├── theme-v3.schema.json
│       ├── target-mapping-v1.schema.json
│       └── target-mapping-v2.schema.json
├── targets/
│   ├── README.md
│   ├── kitty/mapping.jsonc
│   ├── hyprland/mapping.jsonc
│   ├── kvantum/mapping.jsonc
│   └── albert/mapping.jsonc
└── themes/
    ├── README.md
    └── _template/
```

## Data flow

```
theme.jsonc (v1, v2, or v3)  ──validate──►  JSON Schema
      │                                      │
      ▼                                      ▼
  v3: tokens + shim_assign            targets/<id>/mapping.jsonc
      → resolved shim colors        +  (v2: shim_to_native)
  v2: tokens + canonical_assign          (v1: canonical_to_native)
      → resolved canonical colors
      │
      └──────────────────────────►  adapters → per-target configs
```

## Technology stack

- **Rust** — CLI and adapters.
- **Nix** — dev shell; future packaging.
- **JSON Schema** — theme + target-mapping; shim catalog ([`shim-colors.md`](specs/shim-colors.md)); canonical key catalog for v2 ([`canonical-colors.md`](specs/canonical-colors.md)).

## Key decisions

- **Theme path** — **`themes/<id>/theme.jsonc`**; **`metadata.name`** = **`<id>`**; **`schema_version`** **`"1"`**, **`"2"`**, or **`"3"`**.
- **v3 palette** — freeform **`tokens`**; semantics via **`shim_assign`**; targets use **`shim_to_native`**.
- **v2 palette** — freeform **`tokens`**; semantics via **`canonical_assign`**; targets use **`canonical_to_native`**.
- **v1 palette** — Base16 slots; optional embedded **`targets`**.
- **`CHROMAMANCER_TARGETS_DIR`** — root for **`targets/`** tree (default: repo root).
- **Apply model** — **apply-quick** vs **apply-nix**; Nix **switch** wins on overlap.

## Future considerations

- Watch mode / reload helpers.
- Tighter schemas per emitted format.
- CI: validate `themes/*` (v1, v2, v3), `targets/*/mapping.jsonc`, **`metadata.name`** vs directory, `cargo test`.
