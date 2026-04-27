# Logic hook registry (v1)

Builtin adapters may expose imperative steps referenced from **`targets.<id>.logic`** in `theme.jsonc`.

**Format:** implementation-defined string ids; document each id’s **adapter version** (e.g. hyprland v1) so themes stay reproducible.

| Target id   | `logic` id | Description |
|-------------|------------|-------------|
| —           | —          | *No builtin hooks registered yet.* Add a row when an adapter implements a named `logic` step. |

**Rules**

- Unknown **`logic`** id for a target → **hard error** (fail validation or apply, adapter choice; prefer fail-fast).
- When introducing a hook, update this table and add tests.

## Target ids (schema `propertyNames`)

These keys are allowed under **`targets`** in **`theme-v1.schema.json`**:

`hyprland`, `kitty`, `gtk`, `qt`, `kvantum`, `quickshell`, `albert`

Adding a new target requires: adapter code, SPEC roadmap row, **schema enum update**, and this list.
