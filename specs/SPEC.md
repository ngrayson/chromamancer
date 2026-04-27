# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/` (v1: **`theme-v1.schema.json`**) and [`specs/logic-registry.md`](logic-registry.md). Bump **`metadata.schema_version`** and/or the schema file when breaking theme shape.
2. **Theme packs** live under **`themes/<theme-id>/`**. Each pack has **`theme.jsonc`** (JSONC) validated after parse against **`theme-v1.schema.json`**.
3. **One theme file** holds **palette + typography + assets** and optional **`targets`**: per-target **`mappings`**, optional **`apply_quick`** / **`apply_nix`**, optional **`logic`**, optional **`overrides`**.
4. **Builtin adapters** (Rust) supply **defaults**, **mode routing** (`apply-quick` vs `apply-nix`), **`logic`** hooks, and **merge** rules documented below.
5. **Apply model:** **`apply-quick`** vs Nix **`switch`**; see **CLI** and **Authoritative paths** below.
6. **Bootstrap:** standalone CLI; no required in-repo flake/HM module (`nix/modules/README.md`).

## Colors (v1): Base16

**Encoding:** every `tokens.*` is **`#RRGGBBAA`**.

**Keys:** `base00`–`base0F`, required. Semantics: [Base16 styling guide](https://github.com/chriskempson/base16/blob/main/styling.md); Qt/Kvantum hints in the table (unchanged from prior revisions).

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

## Metadata (v1)

- **`metadata.schema_version`:** required **`"1"`** for this schema; future majors change this and the schema filename.
- **`metadata.name`:** required **kebab-case** string. If the pack path is **`themes/<id>/theme.jsonc`**, then **`metadata.name` MUST equal `<id>`** so discovery, CI, and docs agree. For a loose `theme.jsonc` loaded only via **`--theme`**, name is still required and should match your chosen id if you mirror into `themes/` later.

## Theme document (`theme.jsonc`)

**Required:** `metadata`, `tokens`, `fonts`. **Optional:** `assets`, `targets`.

### Discovery and CLI inputs

| Mechanism | Use |
|-----------|-----|
| **`--theme <path>`** | Path to any **`theme.jsonc`**. **Pack root** for resolving relative **`assets`** = **directory containing that file**. |
| **`--pack <id>`** (or equivalent) | Resolve **`themes/<id>/theme.jsonc`** relative to **current working directory**, or relative to **`CHROMAMANCER_THEMES_DIR`** if set. Exact flag names TBD in implementation; behavior is normative. |

Implementations SHOULD error if **`--pack`** resolves outside the intended themes root after canonicalization (no `..` escape).

### Registered `targets` keys

Only these property names are valid under **`targets`** (see [`theme-v1.schema.json`](schemas/theme-v1.schema.json) `propertyNames` and [`logic-registry.md`](logic-registry.md)):

`hyprland`, `kitty`, `gtk`, `qt`, `kvantum`, `quickshell`, `albert`

Unknown keys **fail JSON Schema validation**. Adding a target = code + SPEC + **schema enum** + logic-registry.

### Per-target object (`targets.<id>`)

| Key | Purpose |
|-----|---------|
| **`mappings`** | Declarative map; must match **adapter’s documented intermediate shape**. |
| **`apply_quick`** | Used only for **`apply-quick`**. |
| **`apply_nix`** | Used only for **`apply-nix`**. |
| **`logic`** | Builtin hook id; see **`logic-registry.md`**. Unknown id → **fail fast**. |
| **`overrides`** | Deep-merged onto the **final rendered** config for that target (after mappings + logic). |

**Mode skip rule:** If an adapter does **not** support a mode (**quick** or **nix**) and the theme provides **no** enabling block for that mode (`apply_quick` / `apply_nix`), the CLI **SHOULD skip** that target with **non-zero exit** or **clear stderr** (implementation choice: default **warn and continue** for multi-target applies; document in CLI `--help`).

### Mappings vs builtin defaults (merge)

For each target `T`, adapters maintain a **documented intermediate structure** `M_builtin` (from Base16 + fonts + assets defaults).

1. If **`targets.T.mappings`** is **absent**, use `M_builtin` only.
2. If **present**, **deep-merge** `mappings` **onto** `M_builtin`: for each key, if both values are **objects**, recurse; otherwise the **theme value replaces** the builtin value (same as **`overrides`** semantics).
3. Run **`logic`** hook if set (adapter-defined input/output on merged mapping + full theme).
4. Render to **final config representation** (fragment text, nested dict, etc.).
5. Deep-merge **`targets.T.overrides`** onto that render.

## Builtin adapters (summary)

- Version adapters in code (e.g. `hyprland_v1`).
- **Route** **`apply-quick`** → live paths; **`apply-nix`** → user Nix tree only.

### `overrides` merge (unchanged)

Objects: recursive deep merge. Arrays / scalars from `overrides`: **replace** whole value. Unknown keys: forward/strip/error **at emit** per adapter.

## Authoritative paths (quick vs Nix)

- **`apply-quick`** writes **directly** to paths apps read (unless redirected by flags).
- **`apply-nix`** writes only into **your Nix source tree**; **live** paths update when **`nixos-rebuild switch`** / **`home-manager switch`** installs them.
- **If the same path is both quick-written and Nix-managed**, **`switch`** **overwrites** quick output—treat quick as **ephemeral** for those paths.

**Standalone:** with only the CLI, nothing overwrites quick output until you run Nix or another tool.

## CLI: apply modes

| Command | Writes | Live targets |
|---------|--------|--------------|
| **`chromamancer apply-quick`** | Live paths (per adapter + `apply_quick`). | Immediately. |
| **`chromamancer apply-nix`** | Nix config tree only (`apply_nix`). | After **`switch`**. |

**`apply-nix` output root:** user **`--out`** / **`CHROMAMANCER_NIX_OUT`** (names TBD).

Shared flags (conceptual): **`--theme`**, **`--pack`**, target filter, dry-run.

## Reference Nix (user-owned, no in-repo HM module)

Use a **`runCommand`** (or packaged **`chromamancer`** in **`nativeBuildInputs`**) to regenerate files under e.g. `./generated/` consumed by **`home.file`**:

```nix
# Sketch only — adjust paths and chromamancer package.
chromamancerOut = pkgs.runCommand "chromamancer-generated" { nativeBuildInputs = [ chromamancerPkg ]; } ''
  mkdir -p $out
  chromamancer apply-nix \
    --theme ${./themes/my-theme/theme.jsonc} \
    --out $out
    # or: --pack my-theme with CHROMAMANCER_THEMES_DIR set
'';

# home.file = { ".config/hypr/chromamancer.conf".source = "${chromamancerOut}/hypr/..."; };
```

Reproducible themes should use **pack-relative assets**; pin **chromamancer** version.

## Supported targets (roadmap)

| Target     | Status   | Typical class |
|------------|----------|---------------|
| Hyprland   | Planned  | usually fast |
| Kitty      | Planned  | usually fast |
| GTK        | Planned  | often rebuild-only |
| Qt         | Planned  | often rebuild-only |
| Kvantum    | Planned  | often rebuild-only |
| Quickshell | Planned  | TBD |
| Albert     | Planned  | TBD |

## Fonts (v1)

**`fonts.ui.family`**, **`fonts.mono.family`** only. Sizes: HM / per-app / prefs. **v1** does not schema **per-target font overrides**; if needed later, add under **`targets.<id>`** with adapter support.

## Theme pack layout

- **`themes/<id>/theme.jsonc`** — canonical; **`metadata.name`** = **d='id'** (`<id>` is kebab-case folder name).
- **`assets/`** optional beside file.
- Example copy: [`themes/_template/theme.example.jsonc`](../themes/_template/theme.example.jsonc).

**Nix `fromJSON`:** strip JSONC comments or generate strict JSON in CI.

## Assets

- **Preferred:** paths **relative to pack root** (directory of `theme.jsonc`).
- **Absolute** paths: allowed for **local** `apply-quick`; **Nix builds** SHOULD use **pack-relative** paths for reproducibility.

## Security and path safety

- Treat third-party **`theme.jsonc`** as **untrusted input**: review before **`apply`**.
- Implementations MUST **normalize paths** and **reject** or **contain** **`..`** so outputs stay under **configured roots** (pack root for assets, output dir for writes).

## See also

- [`specs/schemas/theme-v1.schema.json`](schemas/theme-v1.schema.json)
- [`specs/logic-registry.md`](logic-registry.md)
- [`themes/README.md`](../themes/README.md)
- [`ARCHITECTURE.md`](../ARCHITECTURE.md)
