# Theme packs

Each **theme** is **`themes/<theme-id>/`** (e.g. `themes/izar/`). **`metadata.name`** in `theme.jsonc` **must equal** `<theme-id>`.

## Layout

- **`theme.jsonc`** (required) — **JSONC**.
  - **v3 (preferred):** [`specs/schemas/theme-v3.schema.json`](../specs/schemas/theme-v3.schema.json) — **`metadata.schema_version`: `"3"`**, **`tokens`**, **`shim_assign`** (see [`specs/shim-colors.md`](../specs/shim-colors.md)). Pair with **`targets/<id>/mapping.jsonc`** using **`target-mapping-v2`** (`shim_to_native`). No embedded **`targets`**.
  - **v2:** [`theme-v2.schema.json`](../specs/schemas/theme-v2.schema.json) — **`canonical_assign`** ([`specs/canonical-colors.md`](../specs/canonical-colors.md)); pair with **`target-mapping-v1`** mapping files.
  - **v1 (legacy):** [`theme-v1.schema.json`](../specs/schemas/theme-v1.schema.json) — Base16 **`base00`–`base0F`**, optional **`targets.<id>`** on the theme file.
- **`assets/`** — media referenced from `theme.jsonc` (paths usually relative to pack root).
- **`_template/theme.example.jsonc`** — **v3** starter.
- **`_template/theme.v2.example.jsonc`** — **v2** (canonical) starter.
- **`_template/theme.v1.legacy.jsonc`** — **v1** reference only.
- **`izar/`** — example **v3** pack.

**Per-target color projection** lives in **[`targets/<id>/mapping.jsonc`](../targets/README.md)** (not in the theme file for v2/v3).

## Discovery (normative)

| Input | Resolution |
|-------|--------------|
| **`--theme <path>`** | Any `theme.jsonc`; pack root = its parent directory (for `assets`). |
| **`--pack <id>`** | `themes/<id>/theme.jsonc` relative to **cwd** or **`CHROMAMANCER_THEMES_DIR`**. |
| **`CHROMAMANCER_TARGETS_DIR`** | Root containing **`targets/<id>/`** (see [`targets/README.md`](../targets/README.md)). |

**Target ids:** [`specs/logic-registry.md`](../specs/logic-registry.md).

## Relationship to specs

1. Bump **`theme-v1` / `theme-v2` / `theme-v3` / `target-mapping-v1` / `target-mapping-v2`** when formats change.
2. CI should validate `themes/*/theme.jsonc`, `targets/*/mapping.jsonc`, and assert **`metadata.name`** matches folder name for real packs.

## `_template`

Not an installable theme—copy to create a new pack.
