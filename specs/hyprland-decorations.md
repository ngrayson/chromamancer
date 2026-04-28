# Hyprland target — window decorations (colors)

**Target id:** `hyprland`  
**Mapping:** [`targets/hyprland/mapping.jsonc`](../targets/hyprland/mapping.jsonc) (`shim_to_native`, schema v2)

Emitted file is an **include snippet** (not a full `hyprland.conf`). Example Hyprland main config:

```ini
source = ~/.config/hypr/chromamancer-decorations.conf
```

## Native key format (`shim_to_native` values)

Values are **Hyprland property names** as they appear inside a block:

| Pattern | Output block |
|---------|----------------|
| `col.*` | `general { … }` (borders: `col.active_border`, `col.inactive_border`, `col.nogroup_border`, …) |
| `decoration.shadow.*` | Nested `decoration { shadow { … } }` (e.g. `decoration.shadow.color`) |

**Color syntax:** resolved shims are written as `rgba(rrggbbaa)` (lowercase hex, 8 digits — [Hyprland variables](https://wiki.hyprland.org/Configuring/Variables/)).

**Transparent** shims become `rgba(00000000)` where allowed (e.g. inactive border).

Default mapping uses shared shims (**`chrome_focus`** → focused border, **`dark`** → unfocused). Adjust per theme or fork the mapping.

## Non-color decoration

**Rounding**, **gaps**, **border_size**, shadow **range** / **enabled**, etc. stay in your normal Hyprland / Home Manager config. This target only centralizes **colors** that should track `theme.jsonc`.

## Home Manager

Keep static options in `wayland.windowManager.hyprland.settings` and either:

- `xdg.configFile."hypr/chromamancer-decorations.conf"` from a generated file, or  
- `chromamancer apply-quick hyprland` writing that path before `switch`.
