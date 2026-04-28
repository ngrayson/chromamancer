# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/` — **theme:** [`theme-v1.schema.json`](schemas/theme-v1.schema.json) (legacy), [`theme-v2.schema.json`](schemas/theme-v2.schema.json) (canonical keys), or [`theme-v3.schema.json`](schemas/theme-v3.schema.json) (**preferred:** shims); **targets:** [`target-mapping-v1.schema.json`](schemas/target-mapping-v1.schema.json) (**canonical → native**) or [`target-mapping-v2.schema.json`](schemas/target-mapping-v2.schema.json) (**shim → native**); plus [`specs/shim-colors.md`](shim-colors.md), [`specs/canonical-colors.md`](canonical-colors.md) (v2 catalog), and [`specs/logic-registry.md`](logic-registry.md).
2. **Theme packs** — `themes/<theme-id>/theme.jsonc` (JSONC). **v1:** fixed Base16 `tokens` + optional embedded `targets`. **v2:** arbitrary `tokens`, **`canonical_assign`** → per-stack canonical keys. **v3 (preferred):** **`shim_assign`** → shared semantic shims ([`shim-colors.md`](shim-colors.md)); **no** embedded `targets`.
3. **Target mappings** — `targets/<target-id>/mapping.jsonc`: **v1:** **`canonical_to_native`** + optional `apply_*`; **v2:** **`shim_to_native`** (values may be string or string[] when one shim fans out). Discovered via **`CHROMAMANCER_TARGETS_DIR`** (see [`targets/README.md`](../targets/README.md)).
4. **Builtin adapters** supply merge rules, **`apply-quick`** / **`apply-nix`** routing, and **`logic`** hooks (where used).
5. **Apply model:** **`apply-quick`** vs Nix **`switch`**; see **Authoritative paths** below.
6. **Bootstrap:** standalone CLI; no required in-repo HM module (`nix/modules/README.md`).

## Theme format v3 (preferred, shims)

**Validate:** [`theme-v3.schema.json`](schemas/theme-v3.schema.json).

| Part | Purpose |
|------|---------|
| **`metadata.schema_version`** | **`"3"`** |
| **`tokens`** | Named swatches only; values **`#RRGGBBAA`**; names match `^[a-z][a-z0-9_-]*$`. |
| **`shim_assign`** | Maps **shim ids** (enum in schema; catalog in [`shim-colors.md`](shim-colors.md)) → **token name**, **`#RRGGBB`/`#RRGGBBAA` literal**, or **`transparent`** where allowed. |
| **`fonts`**, **`assets`** | Same as v1/v2. |

**Pipeline:** resolve **`shim_assign`** using **`tokens`** → **shim color table** (hex / transparent). Load **`targets/<id>/mapping.jsonc`** with **`metadata.schema_version`: `"2"`** and **`shim_to_native`**, merge with adapter builtins, emit native configs.

### Theme format v2 (canonical, legacy for new packs)

**Validate:** [`theme-v2.schema.json`](schemas/theme-v2.schema.json).

| Part | Purpose |
|------|--------|
| **`metadata.schema_version`** | **`"2"`** |
| **`tokens`** | Named swatches only; values **`#RRGGBBAA`**; names match `^[a-z][a-z0-9_-]*$`. |
| **`canonical_assign`** | Maps **canonical keys** (enum in schema; catalog in [`canonical-colors.md`](canonical-colors.md)) → **token name**, **`#RRGGBB`/`#RRGGBBAA` literal**, or **`transparent`** (Albert borders). |
| **`fonts`**, **`assets`** | Unchanged from v1 semantics. |

**Pipeline:** resolve **`canonical_assign`** using **`tokens`** → **canonical color table** (hex / transparent). For each target, load **`targets/<id>/mapping.jsonc`** with **`metadata.schema_version`: `"1"`** and **`canonical_to_native`**, merge with adapter builtins, emit native configs.

### Theme format v1 (legacy, Base16)

**Validate:** [`theme-v1.schema.json`](schemas/theme-v1.schema.json). **`metadata.schema_version`** **`"1"`**. Fixed **`base00`–`base0F`** tokens; optional **`targets.<id>`** with `mappings`, `apply_quick`, `apply_nix`, `overrides`, `logic`. New work should use **v3 + `targets/`** (shim mappings); v2 + canonical mappings and v1 remain until packs are migrated.

### Discovery and CLI inputs

| Mechanism | Use |
|-----------|-----|
| **`--theme <path>`** | Path to **`theme.jsonc`**. **Pack root** (for **`assets`**) = **directory containing the file**. |
| **`--pack <id>`** | **`themes/<id>/theme.jsonc`** relative to **cwd** or **`CHROMAMANCER_THEMES_DIR`**. |
| **`CHROMAMANCER_TARGETS_DIR`** | Root containing **`targets/<id>/mapping.jsonc`**; default = repo root (see [`targets/README.md`](../targets/README.md)). |

Implementations SHOULD error if paths resolve outside intended roots after canonicalization (no `..` escape).

### Registered target ids

**Theme v1** `propertyNames` / **target-mapping** `metadata.target_id`: `hyprland`, `kitty`, `gtk`, `qt`, `kvantum`, `quickshell`, `albert`.

Adding a target requires: adapter code, SPEC roadmap row, **schema enum** updates (`target-mapping-v1` / **`target-mapping-v2`**, **`theme-v2` canonical enum** and/or **`theme-v3` shim enum** if new semantic keys), [`shim-colors.md`](shim-colors.md) / [`canonical-colors.md`](canonical-colors.md) as appropriate, [`logic-registry.md`](logic-registry.md), and **`targets/<id>/mapping.jsonc`**.

### Per-target object (theme v1 only)

| Key | Purpose |
|-----|---------|
| **`mappings`** | Declarative map; adapter shape. |
| **`apply_quick`** | **`apply-quick`** only. |
| **`apply_nix`** | **`apply-nix`** only. |
| **`logic`** | Hook id; see **`logic-registry.md`**. |
| **`overrides`** | Deep-merged on final render. |

**Mode skip rule:** If an adapter does not support a mode and the theme provides no enabling block, CLI SHOULD skip with clear stderr (implementation choice: warn and continue for multi-target).

### Mappings vs builtin defaults (theme v1 merge)

For each target `T`, adapters maintain **`M_builtin`**.

1. Absent **`mappings`** → **`M_builtin`** only.
2. Present → **deep-merge** `mappings` onto **`M_builtin`** (objects recurse; scalars/arrays replace).
3. **`logic`** if set.
4. Render to native representation.
5. Deep-merge **`overrides`**.

Theme **v2** and **v3** use repo **`targets/<id>/mapping.jsonc`** + adapter code for projection instead of per-theme `mappings`.

## Colors v1 appendix: Base16 token keys

Legacy **v1** requires **`base00`–`base0F`** with **`#RRGGBBAA`**. Semantics: [Base16 styling guide](https://github.com/chriskempson/base16/blob/main/styling.md).

| Key | Base16 role (short) | Typical Qt / Kvantum / UI use |
|-----|---------------------|-------------------------------|
| `base00` | default background | `QPalette::Window`, main Kvantum window/base fill |
| `base01` | lighter background | `AlternateBase`, status bars, inactive tabs |
| `base02` | selection background | `Highlight` background-tones, list selection plane |
| `base03` | subtle / comments | borders, disabled, placeholder tone |
| `base04` | dark foreground | secondary `WindowText`, dim labels |
| `base05` | default foreground | `QPalette::Text`, primary content |
| `base06` | light foreground | emphasized labels, bright `WindowText` |
| `base07` | light background | popovers, tooltips base, elevated panels |
| `base08` | red / diff delete | destructive, error accents |
| `base09` | orange | constants / URIs emphasis |
| `base0A` | yellow | warning, search highlight, caution |
| `base0B` | green / diff add | success, positive indicators |
| `base0C` | cyan | links, info, quotes in syntax |
| `base0D` | blue | `QPalette::Highlight` / focus, links, “accent” in many themes |
| `base0E` | magenta | secondary accent, keyword emphasis |
| `base0F` | brown / deprecated | rare accents, legacy chrome |

**v2/v3:** authors may still *name* tokens `base00`… for compatibility; **v3** assigns **`palette_0`–`palette_15`** explicitly in **`shim_assign`**.

### Kitty `color0`–`color15` (reference)

Default **base16-shell-style** ANSI mapping for tools that fill terminal slots from Base16 is documented in the v1 template history; **v3** assigns **`palette_0`–`palette_15`** in **`shim_assign`**; **v2** uses **`terminal.color0`–`terminal.color15`** in **`canonical_assign`**.

## Metadata

- **`metadata.schema_version`:** **`"1"`** (v1 theme), **`"2"`** (v2 theme), or **`"3"`** (v3 theme).
- **`metadata.name`:** kebab-case; MUST equal **`themes/<id>/`** folder when packed.

## Builtin adapters (summary)

- Version adapters in code (e.g. `hyprland_v1`).
- **Route** **`apply-quick`** → live paths; **`apply-nix`** → user Nix tree only.

### `overrides` merge (v1)

Objects: recursive deep merge. Arrays / scalars: **replace**. Unknown keys: forward/strip/error at emit per adapter.

## Authoritative paths (quick vs Nix)

- **`apply-quick`** writes **directly** to live paths (unless flags override).
- **`apply-nix`** writes only into **your Nix source tree**; materializes on **`nixos-rebuild switch`** / **`home-manager switch`**.
- Overlap: **Nix wins** for the same path.

## CLI: apply modes

| Command | Writes | Live targets |
|---------|--------|--------------|
| **`chromamancer apply-quick`** | Live paths. | Immediately. |
| **`chromamancer apply-nix`** | Nix tree only. | After **`switch`**. |

**`apply-nix` output root:** **`--out`** / **`CHROMAMANCER_NIX_OUT`**.

Shared flags (conceptual): **`--theme`**, **`--pack`**, target filter, dry-run.

## Reference Nix (user-owned, no in-repo HM module)

Use **`runCommand`** (or packaged **`chromamancer`**) to regenerate files under **`./generated/`** consumed by **`home.file`**:

```nix
chromamancerOut = pkgs.runCommand "chromamancer-generated" { nativeBuildInputs = [ chromamancerPkg ]; } ''
  mkdir -p $out
  chromamancer apply-nix \
    --theme ${./themes/my-theme/theme.jsonc} \
    --out $out
'';
```

Reproducible themes: **pack-relative assets**; pin **chromamancer** version.

## Supported targets (roadmap)

| Target     | Status   | Typical class |
|------------|----------|---------------|
| Hyprland   | Planned  | usually fast |
| Kitty      | Planned  | usually fast |
| GTK        | Planned  | often rebuild-only |
| Qt         | Planned  | often rebuild-only |
| Kvantum    | Planned  | often rebuild-only |
| Quickshell | Planned  | TBD |
| Albert     | Partial  | mapping v1 + manual INI |

## Fonts

**`fonts.ui.family`**, **`fonts.mono.family`** only (all versions). Per-target font overrides: future.

## Theme pack layout

- **`themes/<id>/theme.jsonc`** — **`metadata.name`** = **`<id>`**.
- **`assets/`** optional.
- Templates: **[`themes/_template/theme.example.jsonc`](../themes/_template/theme.example.jsonc)** (v2), **[`theme.v1.legacy.jsonc`](../themes/_template/theme.v1.legacy.jsonc)** (v1 reference).

**Nix `fromJSON`:** strip JSONC comments or emit strict JSON in CI.

## Assets

- **Preferred:** paths **relative to pack root**.
- **Absolute:** OK for local **`apply-quick`**; Nix builds SHOULD use pack-relative paths.

## Security and path safety

- Treat third-party theme/mapping files as **untrusted** until reviewed.
- Normalize paths; **reject** **`..`** escaping configured roots.

## See also

- [`specs/schemas/theme-v1.schema.json`](schemas/theme-v1.schema.json)
- [`specs/schemas/theme-v2.schema.json`](schemas/theme-v2.schema.json)
- [`specs/schemas/theme-v3.schema.json`](schemas/theme-v3.schema.json)
- [`specs/schemas/target-mapping-v1.schema.json`](schemas/target-mapping-v1.schema.json)
- [`specs/schemas/target-mapping-v2.schema.json`](schemas/target-mapping-v2.schema.json)
- [`specs/shim-colors.md`](shim-colors.md)
- [`specs/hyprland-decorations.md`](hyprland-decorations.md)
- [`specs/qt-apps.md`](qt-apps.md)
- [`specs/canonical-colors.md`](canonical-colors.md)
- [`targets/README.md`](../targets/README.md)
- [`specs/logic-registry.md`](logic-registry.md)
- [`themes/README.md`](../themes/README.md)
- [`ARCHITECTURE.md`](../ARCHITECTURE.md)
