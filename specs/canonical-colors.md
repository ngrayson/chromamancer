# Canonical color keys (v2, legacy)

Stable **dotted identifiers** for [`theme-v2.schema.json`](schemas/theme-v2.schema.json) **`canonical_assign`**. **New themes** should prefer **[`theme-v3.schema.json`](schemas/theme-v3.schema.json)** + **[`shim-colors.md`](shim-colors.md)** and **`shim_to_native`** mappings.

## Terminal (Kitty-oriented)

| Canonical key | Typical role |
|---------------|--------------|
| `terminal.background` | Default background |
| `terminal.foreground` | Default foreground |
| `terminal.selection_background` | Selection bg |
| `terminal.selection_foreground` | Selection fg |
| `terminal.cursor` | Cursor color |
| `terminal.cursor_text` | Text under cursor |
| `terminal.url` | URL underline / hint |
| `terminal.color0` … `terminal.color15` | ANSI palette (index = xterm 16-color slot) |

**Note:** Default **base16-shell** ANSI ordering for automations is documented in [SPEC.md](SPEC.md) (v1 appendix); v2 themes assign slots explicitly via `canonical_assign`.

## Qt / Kvantum (`GeneralColors`)

Canonical ids map to **[Kvantum `GeneralColors`](https://github.com/tsujan/Kvantum/blob/master/Kvantum/doc/Theme-Config)** keys as `*.color` in `.kvconfig`:

| Canonical key | Kvantum key |
|---------------|-------------|
| `qt.kvantum.window` | `window.color` |
| `qt.kvantum.base` | `base.color` |
| `qt.kvantum.alt_base` | `alt.base.color` |
| `qt.kvantum.button` | `button.color` |
| `qt.kvantum.light` | `light.color` |
| `qt.kvantum.mid_light` | `mid.light.color` |
| `qt.kvantum.dark` | `dark.color` |
| `qt.kvantum.mid` | `mid.color` |
| `qt.kvantum.highlight` | `highlight.color` |
| `qt.kvantum.inactive_highlight` | `inactive.highlight.color` |
| `qt.kvantum.text` | `text.color` |
| `qt.kvantum.window_text` | `window.text.color` |
| `qt.kvantum.button_text` | `button.text.color` |
| `qt.kvantum.inactive_text` | `inactive.text.color` |
| `qt.kvantum.disabled_text` | `disabled.text.color` |
| `qt.kvantum.tooltip_base` | `tooltip.base.color` |
| `qt.kvantum.tooltip_text` | `tooltip.text.color` |
| `qt.kvantum.highlight_text` | `highlight.text.color` |
| `qt.kvantum.link` | `link.color` |
| `qt.kvantum.link_visited` | `link.visited.color` |

## Albert (Widgets Box Model INI)

### `albert.palette.*` → `[palette]` keys

| Canonical | INI key |
|-----------|---------|
| `albert.palette.base` | `base` |
| `albert.palette.text` | `text` |
| `albert.palette.window` | `window` |
| `albert.palette.window_text` | `window_text` |
| `albert.palette.button` | `button` |
| `albert.palette.button_text` | `button_text` |
| `albert.palette.highlight` | `highlight` |
| `albert.palette.highlight_text` | `highlight_text` |
| `albert.palette.placeholder_text` | `placeholder_text` |
| `albert.palette.link` | `link` |
| `albert.palette.link_visited` | `link_visited` |

### `albert.window.*` → `[window]` keys

Keys match **`Theme.ini.template`** in [albert-plugin-widgetsboxmodel](https://github.com/albertlauncher/albert-plugin-widgetsboxmodel/blob/master/themes/Theme.ini.template) (e.g. `input_background_brush`, `result_item_selection_background_brush`, …).

| Canonical | INI key |
|-----------|---------|
| `albert.window.input_background_brush` | `input_background_brush` |
| `albert.window.input_border_brush` | `input_border_brush` |
| `albert.window.input_trigger_color` | `input_trigger_color` |
| `albert.window.input_hint_color` | `input_hint_color` |
| `albert.window.settings_button_color` | `settings_button_color` |
| `albert.window.settings_button_highlight_color` | `settings_button_highlight_color` |
| `albert.window.result_item_selection_background_brush` | `result_item_selection_background_brush` |
| `albert.window.result_item_selection_border_brush` | `result_item_selection_border_brush` |
| `albert.window.result_item_selection_text_color` | `result_item_selection_text_color` |
| `albert.window.result_item_selection_subtext_color` | `result_item_selection_subtext_color` |
| `albert.window.result_item_text_color` | `result_item_text_color` |
| `albert.window.result_item_subtext_color` | `result_item_subtext_color` |
| `albert.window.action_item_selection_background_brush` | `action_item_selection_background_brush` |
| `albert.window.action_item_selection_border_brush` | `action_item_selection_border_brush` |
| `albert.window.action_item_selection_text_color` | `action_item_selection_text_color` |
| `albert.window.action_item_text_color` | `action_item_text_color` |

**Non-color literals:** `albert.window.result_item_selection_border_brush` and `action_item_selection_border_brush` may use assign value **`transparent`** (not a token).

## Validation

- Every key in **`canonical_assign`** MUST appear in **`theme-v2.schema.json`** `propertyNames` enum (or validation fails).
- Every **token reference** in values MUST exist in **`tokens`**.
- Values are either: **`#RRGGBBAA`**, **`#RRGGBB`** (discouraged; normalize in tooling), **token name**, or **`transparent`** where the schema allows.

## Adding keys

1. Add a row here and in **theme-v2** schema enum.
2. Update relevant **`targets/<id>/mapping.jsonc`** `canonical_to_native`.
3. Bump mapping **`metadata.schema_version`** only if the mapping file shape changes (separate from theme v2).
