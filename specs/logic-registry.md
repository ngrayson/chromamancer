# Logic hook registry (v1)

Builtin adapters may expose imperative steps referenced from **`targets.<id>.logic`** in **theme v1** `theme.jsonc`. Theme **v2** does not embed targets; **`logic`** (if reintroduced) would live in **`targets/<id>/mapping.jsonc`** or adapter config—TBD when hooks exist.

**Format:** implementation-defined string ids; document each id’s **adapter version** (e.g. hyprland v1).

| Target id   | `logic` id | Description |
|-------------|------------|-------------|
| —           | —          | *No builtin hooks registered yet.* |

**Rules**

- Unknown **`logic`** id → **hard error** (fail-fast).
- When introducing a hook, update this table and add tests.

## Target ids

These ids name **adapters** and **`targets/<id>/mapping.jsonc`** folders:

`hyprland`, `kitty`, `gtk`, `qt`, `kvantum`, `quickshell`, `albert`

Listed in **`target-mapping-v1.schema.json`** `metadata.target_id` and (for v1 only) **`theme-v1.schema.json`** `targets.propertyNames`.

Adding a target requires: adapter code, SPEC roadmap row, schema updates, **[`canonical-colors.md`](canonical-colors.md)** + **`theme-v2`** enum if new semantic keys, **`targets/<id>/mapping.jsonc`**, and this list.
