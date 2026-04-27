# Scheme packs

Each **scheme** is a directory under `schemes/<scheme-id>/` (for example `schemes/nord-forest/`).

## Layout (conventional)

- **`scheme.json`** (required at pack root) — **JSON with comments (JSONC)**: `//` and `/* */` allowed; plain JSON is fine. After parsing, the value must match `specs/schemas/scheme-v1.schema.json` (Base16 `tokens`, required `fonts` families—no sizes, `#RRGGBBAA` colors). See **Scheme pack file (v1)** in `specs/SPEC.md`.
- `assets/` — wallpapers, user photo, icons, or other media referenced by the scheme file.

**Discovery:** chromamancer (CLI / Nix helpers) loads **`schemes/<id>/scheme.json`** only—no alternate names or YAML in v1.

## Relationship to specs

1. Bump or extend **`specs/schemas/`** when you need new token keys or target maps.
2. Add or edit schemes **only** with data that validates against the schema (CI / CLI checks to be added).

## `_template`

The `_template` directory is a **non-scheme** placeholder for copying when starting a new pack. Do not treat `_template` as an installable theme.
