# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/`. Version them (`scheme-v1`, later `scheme-v2`) before adding generators or Nix glue.
2. **Schemes are data** under `schemes/<name>/`. They must validate against the active schema (tooling TBD).
3. **Targets** map from the **Base16 core** (`base00`–`base0F`) and **global fonts** — no ad hoc color names in v1.
4. **Iteration**: the Rust CLI under `crates/cli/` will eventually apply schemes quickly; **Nix / Home Manager** applies the same artifacts declaratively once a look is finalized.

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

Generators **may** derive non-palette decoration (e.g. Hyprland border gradients, extra Kvantum SVG shades) from these sixteen values using documented formulas—those derivatives are **not** new canonical keys in v1.

## Fonts (v1): global `fonts`

The scheme carries **which typefaces** belong to the look, not **how big** they are—font sizes stay in Nix/Home Manager options, per-target configs (e.g. Kitty), or personal prefs so DPI and ergonomics do not fight the palette.

**v1 requires both slots:** `fonts.ui` and `fonts.mono` are always present in a valid scheme (JSON Schema enforces this). Qt/GTK-side generators use `ui`; terminal-style targets use `mono`. If you want one typeface everywhere, set both `family` strings to the same value.

- **`fonts.ui.family`** — proportional UI face (GTK, Qt/Kvantum, shell UI text where applicable).
- **`fonts.mono.family`** — monospace face (terminal, code-ish UI).

Each `family` is a **Linux-usable font family string** (usually a Fontconfig family name).

## Targets (initial wish list)

| Target        | Notes |
|---------------|--------|
| Kitty         | Base16 → terminal colors; `fonts.mono.family` (sizes in Kitty config / HM) |
| GTK           | Map palette from Base16; `fonts.ui.family` where applicable |
| Qt / Kvantum  | `QPalette` / Kvantum from Base16 + `fonts.ui.family` |
| Quickshell    | bar + lock screen |
| Albert        | theme / QSS from palette |
| Hyprland      | `general:col.*`, decoration from Base16 mapping |

## Assets

Schemes may reference **wallpapers**, **avatars**, and other images. Paths should be **relative to the scheme directory** or explicitly declared so Nix can copy them into the store.

## Security

Do **not** store secrets in scheme files. Treat schemes as **public** configuration and assets.

## See also

- `specs/schemas/scheme-v1.schema.json` — v1 JSON Schema (Base16 + `#RRGGBBAA` + fonts).
- `schemes/README.md` — layout for scheme packs.
- `ARCHITECTURE.md` — repository layout and data flow.
