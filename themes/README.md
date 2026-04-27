# Theme packs

Each **theme** is **`themes/<theme-id>/`** (e.g. `themes/izar/`). **`metadata.name`** in `theme.jsonc` **must equal** `<theme-id>`.

## Layout

- **`theme.jsonc`** (required) — **JSONC**.
  - **v2 (preferred):** [`specs/schemas/theme-v2.schema.json`](../specs/schemas/theme-v2.schema.json) — **`metadata.schema_version`: `"2"`**, **`tokens`** (arbitrary names, `#RRGGBBAA`), **`canonical_assign`** (see [`specs/canonical-colors.md`](../specs/canonical-colors.md)). No embedded **`targets`**.
  - **v1 (legacy):** [`theme-v1.schema.json`](../specs/schemas/theme-v1.schema.json) — Base16 **`base00`–`base0F`**, optional **`targets.<id>`** on the theme file.
- **`assets/`** — media referenced from `theme.jsonc` (paths usually relative to pack root).
- **`_template/theme.example.jsonc`** — **v2** starter.
- **`_template/theme.v1.legacy.jsonc`** — **v1** reference only.
- **`izar/`** — example **v2** pack.

**Per-target color projection** lives in **[`targets/<id>/mapping.jsonc`](../targets/README.md)**, not in the theme file (v2).

## Discovery (normative)

| Input | Resolution |
|-------|--------------|
| **`--theme <path>`** | Any `theme.jsonc`; pack root = its parent directory (for `assets`). |
| **`--pack <id>`** | `themes/<id>/theme.jsonc` relative to **cwd** or **`CHROMAMANCER_THEMES_DIR`**. |
| **`CHROMAMANCER_TARGETS_DIR`** | Root containing **`targets/<id>/`** (see [`targets/README.md`](../targets/README.md)). |

**Target ids:** [`specs/logic-registry.md`](../specs/logic-registry.md).

## Relationship to specs

1. Bump **`theme-v1` / `theme-v2` / `target-mapping-v1`** when formats change.
2. CI should validate `themes/*/theme.jsonc`, `targets/*/mapping.jsonc`, and assert **`metadata.name`** matches folder name for real packs.

## `_template`

Not an installable theme—copy to create a new pack.
