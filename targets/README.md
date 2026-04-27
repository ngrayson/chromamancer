# Target mappings (`mapping.jsonc`)

Per-app **canonical → native** projection, versioned separately from theme packs.

## Layout

```
targets/
├── README.md
├── kitty/mapping.jsonc
├── kvantum/mapping.jsonc
├── albert/mapping.jsonc
└── …
```

Each file validates as **`specs/schemas/target-mapping-v1.schema.json`** after JSONC parse.

## `CHROMAMANCER_TARGETS_DIR`

| Input | Resolution |
|-------|-------------|
| Env **`CHROMAMANCER_TARGETS_DIR`** | Directory whose children are **`targets/<id>/`** (i.e. set to repo root, not `targets/` itself). |
| Default | **Repository root** next to `themes/` and `specs/` (bootstrap). |
| Packaged CLI | May default to **`$out/share/chromamancer`** or similar. |

Implementations resolve **`targets/<target_id>/mapping.jsonc`** relative to that root.

## File shape (v1)

- **`metadata`:** `{ "schema_version": "1", "target_id": "kitty" }` (must match folder name).
- **`canonical_to_native`:** maps **[canonical key](canonical-colors.md)** → **native key string** (adapter interprets; see per-target README notes in each `mapping.jsonc`).
- **`apply_quick` / `apply_nix`:** optional blobs for CLI defaults (e.g. Albert output paths).

## Pipeline

1. Load **`themes/<id>/theme.jsonc`** → validate **`theme-v1`** or **`theme-v2`**.
2. If **v2:** resolve **`tokens`** + **`canonical_assign`** → canonical hex table.
3. Load **`targets/<id>/mapping.jsonc`** → merge with adapter builtins → emit native config.

## Adding a target

1. Register id in **`target-mapping-v1.schema.json`**, [`logic-registry.md`](../specs/logic-registry.md), SPEC roadmap.
2. Add canonical keys to [`canonical-colors.md`](../specs/canonical-colors.md) + **`theme-v2.schema.json`** enum.
3. Add **`targets/<id>/mapping.jsonc`**.
