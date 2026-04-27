# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/`. Version them (`scheme-v1`, later `scheme-v2`) before adding generators or Nix glue.
2. **Schemes are data** under `schemes/<name>/`. They must validate against the active schema (tooling TBD).
3. **Targets** (Kitty, GTK, Qt, Quickshell, Albert, Hyprland decorations, etc.) map from the **fixed token core** below — no per-target ad hoc color names in v1.
4. **Iteration**: the Rust CLI under `crates/cli/` will eventually apply schemes quickly; **Nix / Home Manager** applies the same artifacts declaratively once a look is finalized.

## Canonical tokens (v1)

**Encoding:** every token value is **`#RRGGBBAA`** (hash + eight hex digits). **No `#RRGGBB` shorthand** in v1 — use explicit `FF` for fully opaque (e.g. `#c0caf5FF`).

**Core:** exactly these keys, all required on every scheme instance (see `specs/schemas/scheme-v1.schema.json`):

| Token | Meaning |
|-------|---------|
| `bg0` | Deepest surface (e.g. default background) |
| `bg1` | Raised surface (e.g. panels, inactive tabs) |
| `bg2` | Further elevated (e.g. popovers, highlights) |
| `fg0` | Primary foreground |
| `fg1` | Secondary / emphasized foreground |
| `fg_muted` | De-emphasized text |
| `border` | Dividers, faint UI chrome |
| `accent` | Primary accent fill |
| `accent_fg` | Text/icons on top of `accent` |
| `error` | Error / destructive emphasis |
| `warning` | Warning emphasis |
| `success` | Success emphasis |
| `selection_bg` | Selection / list highlight background |
| `selection_fg` | Text/icons on `selection_bg` |

Naming or semantics for any key can be revised while we still call this **scheme v1**; if we remove/rename keys, bump to **v2** in schema filename and `$id`.

## Targets (initial wish list)

| Target        | Notes |
|---------------|--------|
| Kitty         | colors, fonts |
| GTK           | theme, accent |
| Qt            | `qt5ct` / `qt6ct` / platform theme |
| Quickshell    | bar + lock screen |
| Albert        | theme / QSS |
| Hyprland      | `general:col.*`, decoration, misc |

## Assets

Schemes may reference **wallpapers**, **avatars**, and other images. Paths should be **relative to the scheme directory** or explicitly declared so Nix can copy them into the store.

## Security

Do **not** store secrets in scheme files. Treat schemes as **public** configuration and assets.

## See also

- `specs/schemas/scheme-v1.schema.json` — v1 JSON Schema (fixed token core, `#RRGGBBAA`).
- `schemes/README.md` — layout for scheme packs.
- `ARCHITECTURE.md` — repository layout and data flow.
