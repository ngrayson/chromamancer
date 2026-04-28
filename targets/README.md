# Target mappings (`mapping.jsonc`)

Per-app projection from theme semantics → native config keys. The file **`metadata.schema_version`** selects the table shape.

## Layout

```
targets/
├── README.md
├── kitty/README.md
├── kitty/mapping.jsonc
├── hyprland/README.md
├── hyprland/mapping.jsonc
├── qt/mapping.jsonc
├── kvantum/mapping.jsonc
├── albert/README.md
├── albert/mapping.jsonc
└── …
```

- **Theme v3** → validate as **`specs/schemas/target-mapping-v2.schema.json`** after JSONC parse (`shim_to_native`).
- **Theme v2** → validate as **`specs/schemas/target-mapping-v1.schema.json`** (`canonical_to_native`).

First-party **`kitty`**, **`hyprland`**, **`qt`**, **`kvantum`**, **`albert`** files in this repo use **v2** (shims) and pair with **theme v3**.

## `CHROMAMANCER_TARGETS_DIR`

| Input | Resolution |
|-------|-------------|
| Env **`CHROMAMANCER_TARGETS_DIR`** | Directory whose children are **`targets/<id>/`** (i.e. set to repo root, not `targets/` itself). |
| Default | **Repository root** next to `themes/` and `specs/` (bootstrap). |
| Packaged CLI | May default to **`$out/share/chromamancer`** or similar. |

Implementations resolve **`targets/<target_id>/mapping.jsonc`** relative to that root.

## File shape

### Target mapping v2 (shims)

- **`metadata`:** `{ "schema_version": "2", "target_id": "kitty" }` (must match folder name).
- **`shim_to_native`:** maps **[shim id](../specs/shim-colors.md)** → **native key string**, or **array of strings** when one resolved shim color must populate multiple native keys (Albert). Native strings are adapter-defined (e.g. Kitty `background`, Kvantum `window.color`, Albert `palette.base`).
- **`apply_quick` / `apply_nix`:** optional blobs for CLI defaults (e.g. Albert output paths).

### Target mapping v1 (canonical)

- **`metadata.schema_version`:** **`"1"`**
- **`canonical_to_native`:** maps **canonical keys** (see [`specs/canonical-colors.md`](../specs/canonical-colors.md)) → native key string.

## Pipeline

1. Load **`themes/<id>/theme.jsonc`** → validate **`theme-v1`**, **`theme-v2`**, or **`theme-v3`**.
2. **v3:** resolve **`tokens`** + **`shim_assign`** → shim hex table.
3. **v2:** resolve **`tokens`** + **`canonical_assign`** → canonical hex table.
4. Load **`targets/<id>/mapping.jsonc`** at the matching mapping schema version → merge with adapter builtins → emit native config.

## Adding a target

1. Register id in **`target-mapping-v1.schema.json`** and **`target-mapping-v2.schema.json`**, [`logic-registry.md`](../specs/logic-registry.md), SPEC roadmap.
2. For **v3:** add shims to [`specs/shim-colors.md`](../specs/shim-colors.md) + **`theme-v3.schema.json`** enum as needed. For **v2:** add canonical keys to [`specs/canonical-colors.md`](../specs/canonical-colors.md) + **`theme-v2.schema.json`** enum.
3. Add **`targets/<id>/mapping.jsonc`** (v1 and/or v2 as appropriate).

## Targets in this repo

| `target_id` | Role | Field notes |
|-------------|------|-------------|
| `kitty` | Terminal colors | [kitty/README.md](kitty/README.md) |
| `hyprland` | Window border / decoration colors | [hyprland/README.md](hyprland/README.md) |
| `qt` | Qt platform palette (`palette.*` logical keys — [`specs/qt-apps.md`](../specs/qt-apps.md)) | — |
| `kvantum` | Kvantum `GeneralColors` | — |
| `albert` | Widgets Box Model INI | [albert/README.md](albert/README.md) |
