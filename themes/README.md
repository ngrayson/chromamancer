# Theme packs

Each **theme** is a directory **`themes/<theme-id>/`** (for example `themes/nord-forest/`).

## Layout (conventional)

- **`theme.jsonc`** (required at pack root) — **JSONC**: palette (`tokens`), `fonts`, optional `assets`, optional **`targets`** (per-target `mappings`, optional `apply_quick` / `apply_nix`, optional `logic`, optional `overrides`). Must match `specs/schemas/scheme-v1.schema.json` after parsing.
- **`assets/`** — wallpapers, avatars, media referenced from `theme.jsonc`.

**Discovery (v1):** `chromamancer` loads **`themes/<id>/theme.jsonc`** only.

## Relationship to specs

1. Bump **`specs/schemas/`** when token keys or `targets.*` shapes evolve.
2. Validate packs in CI / CLI once tooling exists.

## `_template`

Placeholder for copying when starting a new pack—not an installable theme.
