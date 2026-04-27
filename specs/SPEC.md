# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, theme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/`. Version them (`theme` / `scheme-v1` today, later `v2`) before hard-coding generators.
2. **Theme packs** live under **`themes/<theme-id>/`**. Each pack has a single **`theme.jsonc`** (see below) validated after JSONC parse.
3. **One theme file** holds **palette + typography + assets** and **per-target configuration**: mappings from this theme to each target, plus **optional** `apply_quick` / **`apply_nix`** sections (either or both per target) and optional imperative **`logic`** hooks.
4. **Builtin target adapters** in chromamancer (Rust) provide **defaults**, **mode routing** (`apply-quick` vs `apply-nix`), and **logic** that cannot live in data. The theme file **extends or overrides** declaratively; we add targets by shipping adapters **and** documenting the `targets.<id>` shape.
5. **Iteration vs system of record:** see **Apply model** — `apply-quick` vs Nix **`switch`**.
6. **Bootstrap:** standalone CLI; no required flake/HM module in-repo (see earlier sections / `nix/modules/README.md`).

## Colors (v1): Base16 canonical names

**Why Base16:** Sixteen stable keys match terminal/editor ecosystems and give one row of swatches for generators to map onto **Qt `QPalette` roles**, **Kvantum**-style widgets, GTK, and Wayland compositor colors.

**Encoding:** every `tokens.*` value is **`#RRGGBBAA`** (no `#RRGGBB` shorthand in v1).

**Keys:** exactly `base00` … `base0F`, all required. Meanings follow the [Base16 styling guide](https://github.com/chriskempson/base16/blob/main/styling.md). The table below adds a **Qt / UI-oriented hint** so implementers know *typical* mapping targets (not exhaustive—Kvantum SVG themes may derive extra stops from these).

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

**Swapping themes:** replace the pack (`tokens`, `fonts`, `assets`, `targets` as needed). Builtin adapters stay versioned in the tool.

## Theme document (`theme.jsonc`) and `targets`

**Canonical path:** `themes/<theme-id>/theme.jsonc` (JSONC on disk; `.jsonc` suffix documents intent—parser treats as JSON-with-comments).

**Required top-level:** `metadata`, `tokens`, `fonts`. **Optional:** `assets`, **`targets`**.

### `targets` object

Keys are **target ids** (`hyprland`, `kitty`, `gtk`, …). Each value is an object that may include:

| Key | Purpose |
|-----|---------|
| **`mappings`** | Declarative map from **Base16 / theme slots** into this target’s **intermediate config model** (shape is **per-adapter**; JSON Schema will tighten over time). |
| **`apply_quick`** | Optional block used **only** when running **`chromamancer apply-quick`** (e.g. path hints, fragment names). **Omit** → adapter defaults for quick mode. |
| **`apply_nix`** | Optional block used **only** when running **`chromamancer apply-nix`** (e.g. relative paths inside your Nix tree). **Omit** → adapter defaults for nix mode. |
| **`logic`** | Optional string id referencing **builtin imperative** behavior (transforms the adapter implements when data alone is not enough). Format **per adapter**. |
| **`overrides`** | Optional object **deep-merged** over the fully rendered target config (after mappings + builtin logic). Same merge rules as below. |

**Per target, both `apply_quick` and `apply_nix` may exist, one may exist, or neither** (then the adapter’s defaults define what each command does—for example **rebuild-only** targets might only consult `apply_nix` or ignore quick). **`apply-quick`** / **`apply-nix`** commands skip a target if the adapter deems that mode unsupported **and** the theme provides no enabling block—exact behavior per adapter, documented in tests.

### Builtin adapters (in chromamancer)

- Implement **default mappings** and **logic** when `mappings` / `logic` are absent or partial.
- **Version** adapters (e.g. `hyprland_v1`) so old `theme.jsonc` files keep working.
- **Route** output to **live target files** (`apply-quick`) vs **Nix tree only** (`apply-nix`).
- Add new targets by adding an adapter + documenting the `targets.<id>` contract.

### `targets.<id>.overrides` merge semantics (v1)

**Merge order** for target `T`: start from the adapter’s **rendered config** (tokens + fonts + assets + **`mappings`** + **`logic`**), then deep-merge **`overrides`**. **Override wins** on key conflicts.

**Objects:** recursive deep merge.

**Non-objects (including arrays):** replace whole value from `overrides`.

**Unknown keys:** merged in; **emit-time** strictness is per adapter.

## Apply model: fast iteration vs Nix

**Standalone bootstrap:** if you **only** run the chromamancer CLI, it is the **sole** writer to the paths you configure—nothing automatically overwrites its output until **you** introduce another mechanism (Nix, another tool, etc.).

**When you use Nix for the same paths:** **`nixos-rebuild switch`** / **`home-manager switch`** installs from your config and **overwrites** live paths that Nix manages.

**Two classes of target (typical):**

1. **Fast-iterative** — `apply-quick` writes **directly** to files the app reads; reload when possible.
2. **Rebuild-only** — sensible output is **`apply-nix`** into your flake tree; live paths update on **`switch`**, not from quick.

**Authoritative Nix (when you opt in):** same as before—Nix activation owns files it installs.

## CLI: apply modes (bootstrap)

They share the **same pipeline** (`theme.jsonc` + adapters + merge).

| Command | Writes to | Live target files |
|---------|-----------|-------------------|
| **`chromamancer apply-quick`** | **Directly** to paths each running target reads (per adapter, using `apply_quick` from `theme.jsonc` when present). | Immediately; reload/restart as needed. |
| **`chromamancer apply-nix`** | **Only** your **Nix configuration tree** (using `apply_nix` when present). | After **`switch`**, Nix materializes into target paths. |

Output root for **`apply-nix`** remains **user-configured** (`--out` / env—names TBD).

**Workflow:** **`apply-quick`** for iteration; **`apply-nix`** + commit + **`switch`** for durable system state.

Shared flags (conceptual): path to **`theme.jsonc`**, target selection, dry-run—TBD in implementation.

## Supported targets (adapter roadmap)

| Target        | Role of adapter | Typical iteration class |
|---------------|-----------------|-------------------------|
| Kitty         | Map Base16 + `targets.kitty` → terminal + font family | usually **fast** |
| GTK           | Palette / fragments | often **rebuild-only** |
| Qt / Kvantum  | QPalette / Kvantum-related | often **rebuild-only** |
| Quickshell    | Bar / lock | **TBD** |
| Albert        | QSS / theme | **TBD** |
| Hyprland      | `general:col.*`, decoration | usually **fast** |

## Fonts (v1): global `fonts`

Same as before: **`fonts.ui.family`**, **`fonts.mono.family`** only (no sizes in theme). Sizes in HM / per-app config / prefs.

## Theme pack layout

- **Directory:** `themes/<theme-id>/`.
- **File:** **`theme.jsonc`** (required) — JSONC; validates against [`specs/schemas/scheme-v1.schema.json`](specs/schemas/scheme-v1.schema.json) on the **parsed JSON** value.
- **Assets:** optional `assets/` beside the file; paths in `assets` are relative to the pack root unless stated otherwise.
- **Discovery:** CLI loads **`themes/<id>/theme.jsonc`** only (v1).

**Nix note:** `builtins.fromJSON` needs strict JSON; use a build step or export without comments when importing from Nix.

## Assets

Theme packs may reference **wallpapers**, **avatars**, etc.; paths **relative to the pack directory** (or explicit) so Nix can copy into the store when needed.

## Security

Do **not** store secrets in `theme.jsonc`. Treat themes as **public** configuration and assets.

## See also

- `specs/schemas/scheme-v1.schema.json` — v1 JSON Schema (theme document).
- `themes/README.md` — pack layout.
- `ARCHITECTURE.md` — repo layout and data flow.
