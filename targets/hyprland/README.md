# Hyprland target (window decoration colors)

**Status:** Validated **manually** (Izar border colors in Home Manager + `lib.mkForce` over Stylix). CLI `apply-quick hyprland` exists in-repo; the generated snippet should match what you maintain by hand once the same mapping is used.

## Contract

- **Mapping:** [`mapping.jsonc`](mapping.jsonc) — `chrome_focus` → `col.active_border`, `dark` → `col.inactive_border`. Optional keys may use the `decoration.shadow.*` prefix (nested block); see [`../../specs/hyprland-decorations.md`](../../specs/hyprland-decorations.md).
- **Output shape:** Hyprland `rgba(rrggbbaa)` assignments inside `general { }` (and optional `decoration { shadow { … } }`).

## What has worked in practice

- **NixOS / Home Manager:** `wayland.windowManager.hyprland.settings.general` with quoted keys `"col.active_border"` / `"col.inactive_border"`.
- **Stylix:** Also sets `general.col.*` (e.g. Tokyo Night). Those definitions **conflict** at evaluation time unless Nix merge priority is set — **`lib.mkForce`** on the Izar values has been used so borders track **Izar** (`themes/izar`: `chrome_focus` = teal, `dark` = plum).
- **Rounding / gaps / border_size:** Still owned by HM static settings; this target only covers **colors**.

## Automation TODO

- `chromamancer apply-quick hyprland -t themes/izar/theme.jsonc` → default `~/.config/hypr/chromamancer-decorations.conf`.
- HM/Nix: `source = …/chromamancer-decorations.conf` near the end of the effective Hyprland config, or keep using `mkForce` in `settings` if you prefer one generated file-less path.

## Pitfalls

- Merge order: any other module that sets `col.active_border` / `col.inactive_border` needs **`mkForce`**, later `source`, or Stylix `targets.hyprland` disabled if you want full manual control.
- **Izar:** Focused border uses **`chrome_focus`** (teal); unfocused uses **`dark`** (plum). Keep `mapping.jsonc` aligned with that choice.
