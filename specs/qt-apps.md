# Qt apps target (`qt`)

**Target id:** `qt`  
**Mapping:** [`targets/qt/mapping.jsonc`](../targets/qt/mapping.jsonc)

This target is for **Qt widget colors** on the **platform palette** (Fusion, Breeze without full KDE theme, **`qt5ct`** / **`qt6ct`**, and optionally **`kdeglobals`** / Plasma later). It is **separate from** [`targets/kvantum/mapping.jsonc`](../targets/kvantum/mapping.jsonc): Kvantum drives **Kvantum SVG themes**; **`qt`** drives **QPalette-style** colors for apps that respect the platform palette or qt\*ct.

**No emitters ship yet** — only the contract (this doc + `shim_to_native`). Application logic will map `palette.*` strings to real files (`qt5ct` color schemes, `kdeglobals` `[Colors:*]`, etc.).

## Native keys (`shim_to_native` values)

Values are **logical identifiers** with the prefix **`palette.`** + **snake_case** role name aligned with [Qt `QPalette::ColorRole`](https://doc.qt.io/qt-6/qpalette.html#ColorRole-enum) (active group unless an adapter documents otherwise).

| Native id | Typical `QPalette::ColorRole` |
|-----------|------------------------------|
| `palette.window` | `Window` |
| `palette.window_text` | `WindowText` |
| `palette.base` | `Base` |
| `palette.alternate_base` | `AlternateBase` |
| `palette.text` | `Text` |
| `palette.button` | `Button` |
| `palette.button_text` | `ButtonText` |
| `palette.light` | `Light` |
| `palette.midlight` | `Midlight` |
| `palette.dark` | `Dark` |
| `palette.mid` | `Mid` |
| `palette.shadow` | `Shadow` |
| `palette.highlight` | `Highlight` |
| `palette.highlighted_text` | `HighlightedText` |
| `palette.link` | `Link` |
| `palette.link_visited` | `LinkVisited` |
| `palette.tool_tip_base` | `ToolTipBase` |
| `palette.tool_tip_text` | `ToolTipText` |
| `palette.placeholder_text` | `PlaceholderText` |
| `palette.accent` | `Accent` (Qt 6+) |
| `palette.secondary_highlight` | *Logical* — unfocused / muted emphasis (maps to inactive `Highlight` or engine-specific “inactive.highlight”) |
| `palette.muted_text` | *Logical* — de-emphasized labels (maps to `PlaceholderText`, inactive `WindowText`, or engine-specific “inactive.text”) |

Adapters may duplicate one shim into **active / inactive / disabled** groups when the output format requires it.

## Relation to shims

Uses the same [semantic shims](shim-colors.md) as Kvantum where roles align (e.g. `chrome_focus` → `palette.highlight`, `list_selection` → `palette.accent`). Terminal-only shims (`palette_0`…`palette_15`, cursor, …) are **not** listed in the default `targets/qt/mapping.jsonc`.

## Adding output formats

1. Extend this table if new **logical** roles are required.  
2. Prefer reusing existing shims; add shims in **`theme-v3`** + **`shim-colors.md`** only when a role is truly new.  
3. Document the file syntax (qt5ct INI, `kdeglobals`, etc.) in this file when an adapter is implemented.
