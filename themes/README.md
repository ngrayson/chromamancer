# Theme packs

Each **theme** is **`themes/<theme-id>/`** (e.g. `themes/nord-forest/`). **`metadata.name`** in `theme.jsonc` **must equal** `<theme-id>`.

## Layout

- **`theme.jsonc`** (required) — **JSONC**. Validate parsed JSON against [`specs/schemas/theme-v1.schema.json`](../specs/schemas/theme-v1.schema.json) (requires **`metadata.schema_version`: `"1"`**).
- **`assets/`** — media referenced from `theme.jsonc` (paths usually relative to pack root).
- **`_template/theme.example.jsonc`** — starting point; copy to **`theme.jsonc`** in a new pack.
- **`izar/`** — example pack (Izar palette: Kitty/Kvantum mappings + Albert `apply_nix` stub).

## Discovery (normative)

| Input | Resolution |
|-------|--------------|
| **`--theme <path>`** | Any `theme.jsonc`; pack root = its parent directory (for `assets`). |
| **`--pack <id>`** | `themes/<id>/theme.jsonc` relative to **cwd** or **`CHROMAMANCER_THEMES_DIR`**. |

Registered **`targets.*`** keys: [`specs/logic-registry.md`](../specs/logic-registry.md).

## Relationship to specs

1. Bump **`theme-v1.schema.json`** / **`metadata.schema_version`** when the format changes.
2. CI should validate `themes/*/theme.jsonc` + assert **`metadata.name`** matches folder name.

## `_template`

Not an installable theme—copy to create a new pack.
