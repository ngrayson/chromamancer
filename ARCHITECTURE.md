# Architecture — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## Overview

chromamancer is a **Rust workspace**, **specs** + JSON Schema, **`themes/`** packs (**`theme.jsonc`**), repo-level **`targets/<id>/mapping.jsonc`**, and an optional **Nix devShell**. **Bootstrap** is a **standalone CLI**.

**Theme v2 (preferred):** arbitrary named **`tokens`** (`#RRGGBBAA`) and **`canonical_assign`** → stable semantic keys ([`specs/canonical-colors.md`](specs/canonical-colors.md)). **No** embedded per-app `targets` on the theme.

**Theme v1 (legacy):** fixed Base16 `base00`–`base0F` + optional **`targets.<id>`** (`mappings`, `apply_quick`, `apply_nix`).

**Target mappings:** **`targets/<id>/mapping.jsonc`** defines **canonical → native** projection and optional **`apply_nix`** defaults (see [`targets/README.md`](targets/README.md)). Discovered via **`CHROMAMANCER_TARGETS_DIR`** (repo root by default).

**Builtin adapters** resolve **theme → canonical table → native artifacts** for **`apply-quick`** / **`apply-nix`**. First-party HM/NixOS modules are **deferred**—`nix/modules/README.md`.

## Pillars

1. **Fast iteration — `chromamancer apply-quick`** writes live paths; **`apply-nix`** writes under your Nix tree; **`switch`** materializes (`specs/SPEC.md`).
2. **Nix (optional)** — `nix/flake.nix` is **devShell**; rebuild overwrites overlapping quick output.
3. **Schematic contracts** — `specs/schemas/` (`theme-v1`, **`theme-v2`**, **`target-mapping-v1`**); [`SPEC.md`](specs/SPEC.md) + [`canonical-colors.md`](specs/canonical-colors.md).
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
│       └── target-mapping-v1.schema.json
├── targets/
│   ├── README.md
│   ├── kitty/mapping.jsonc
│   ├── kvantum/mapping.jsonc
│   └── albert/mapping.jsonc
└── themes/
    ├── README.md
    └── _template/
```

## Data flow

```
theme.jsonc (v1 or v2)  ──validate──►  JSON Schema
      │                                      │
      ▼                                      ▼
  v2: tokens + canonical_assign        targets/<id>/mapping.jsonc
      → resolved canonical colors   +    (canonical → native)
      │
      └──────────────────────────►  adapters → per-target configs
```

## Technology stack

- **Rust** — CLI and adapters.
- **Nix** — dev shell; future packaging.
- **JSON Schema** — theme + target-mapping; canonical key catalog in markdown.

## Key decisions

- **Theme path** — **`themes/<id>/theme.jsonc`**; **`metadata.name`** = **`<id>`**; **`schema_version`** **`"1"`** or **`"2"`**.
- **v2 palette** — freeform **`tokens`**; semantics via **`canonical_assign`** only.
- **v1 palette** — Base16 slots; optional embedded **`targets`**.
- **`CHROMAMANCER_TARGETS_DIR`** — root for **`targets/`** tree (default: repo root).
- **Apply model** — **apply-quick** vs **apply-nix**; Nix **switch** wins on overlap.

## Future considerations

- Watch mode / reload helpers.
- Tighter schemas per emitted format.
- CI: validate `themes/*` (v1+v2), `targets/*/mapping.jsonc`, **`metadata.name`** vs directory, `cargo test`.
