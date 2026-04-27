# Semantic shims (v3)

**Shims** are stable **role names** (snake_case). Themes assign **`tokens` → shims** via **`shim_assign`**. Targets map **`shim → native`** keys in **`targets/<id>/mapping.jsonc`** (`shim_to_native`).

Flow: **`tokens`** (author-defined swatches) → **`shim_assign`** (role → token or literal) → **[resolved shim table]** → **`shim_to_native`** per target → config / INI.

v2 **`canonical_assign`** (`terminal.*`, `qt.kvantum.*`, …) is **legacy**; v3 keeps one shared vocabulary for humans (Background, selection, blue, …) instead of per-stack canonical ids.

## Core surfaces & text

| Shim | Typical meaning |
|------|-----------------|
| `background` | Main/default surface (terminal bg, launcher window, often Kvantum `window`) |
| `foreground` | Primary text on `background` |
| `selection_background` | Selected text range bg (terminal) |
| `selection_foreground` | Selected text range fg |
| `cursor` | Insertion caret |
| `cursor_text` | Character under caret |
| `url` | URLs / hints |

## Terminal palette (16 slots)

| Shim | Role |
|------|------|
| `palette_0` … `palette_15` | ANSI / xterm color table (`color0`…`color15` in Kitty). Convention may follow [base16-shell](https://github.com/chriskempson/base16-shell); themes set explicitly. |

## Qt-style chrome (Kvantum `GeneralColors` and similar)

| Shim | Typical Kvantum mapping |
|------|-------------------------|
| `window` | `window.color` |
| `base` | `base.color` |
| `alt_base` | `alt.base.color` |
| `button` | `button.color` |
| `light` | `light.color` |
| `mid_light` | `mid.light.color` |
| `dark` | `dark.color` |
| `mid` | `mid.color` |
| `chrome_focus` | Focus ring / primary highlight (`highlight.color`) — often **distinct** from list row accent |
| `chrome_focus_muted` | Inactive / muted focus (`inactive.highlight.color`) |
| `text` | `text.color` |
| `window_text` | `window.text.color` |
| `button_text` | `button.text.color` |
| `inactive_text` | `inactive.text.color` |
| `disabled_text` | `disabled.text.color` |
| `tooltip_base` | `tooltip.base.color` |
| `tooltip_text` | `tooltip.text.color` |
| `chrome_focus_text` | `highlight.text.color` (text on `chrome_focus`; Albert row text uses `list_selection_foreground`) |
| `link` | `link.color` |
| `link_visited` | `link.visited.color` |
| `placeholder_text` | `placeholder_text` / subdued hints |

## Lists & launcher (Albert / list views)

| Shim | Typical use |
|------|-------------|
| `list_selection` | Accent / fill for **selected list row** (Albert `palette.highlight`, input borders; may differ from `chrome_focus`) |
| `list_subtext` | Secondary line on **non-selected** list rows |
| `list_selection_foreground` | Primary text on the **selected** row |
| `list_selection_subtext` | Secondary line on the **selected** row |
| `selection_border` | Border around selection; value may be **`transparent`** |

## Literals on shims

**`shim_assign`** values follow **`theme-v3`** schema: token name, `#RRGGBBAA`, `#RRGGBB`, or **`transparent`** where allowed (e.g. `selection_border`).

## Adding shims

1. Add a row here and the **enum** in [`theme-v3.schema.json`](schemas/theme-v3.schema.json).
2. Update each **`targets/<id>/mapping.jsonc`** **`shim_to_native`** that consumes the new role.
3. Bump **target mapping** `metadata.schema_version` only if the **file shape** changes (see [`target-mapping-v2.schema.json`](schemas/target-mapping-v2.schema.json)).

**Many-to-one:** one shim may list **several** native paths (JSON array in `shim_to_native`) when multiple INI/config keys should share the same resolved color.

## Relation to v2 canonical keys

[`canonical-colors.md`](canonical-colors.md) documents the old **per-stack** canonical ids. New work should use **shims + v3**; adapters may translate resolved shims into canonical internally if useful.
