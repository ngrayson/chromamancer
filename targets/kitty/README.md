# Kitty target

**Status:** Validated **manually** (Izar colors in a real NixOS / Home Manager setup). CLI `apply-quick kitty` exists in-repo; automation can converge on the same output as the hand-maintained file.

## Contract

- **Mapping:** [`mapping.jsonc`](mapping.jsonc) — `shim_to_native` maps semantic shims → Kitty option names (`background`, `color0`…`color15`, …).
- **Spec:** Terminal shims in [`../../specs/shim-colors.md`](../../specs/shim-colors.md); keys follow [Kitty color options](https://sw.kovidgoyal.net/kitty/conf/#color).

## What has worked in practice

- **Source of truth (manual path):** A small include file (e.g. `izar.conf`) with `background`, `foreground`, selection, cursor, URL, and `color0`–`color15`, deployed via Home Manager `xdg.configFile` from a NixOS config repo.
- **Main config:** `kitty.conf` uses `include izar.conf` (or equivalent) so fonts, padding, and behavior stay separate from generated colors.
- **Theme:** [`themes/izar/theme.jsonc`](../../themes/izar/theme.jsonc) — `shim_assign` for the same roles the mapping lists; hex values match the manual Kitty file when copied from Izar tokens.

## Automation TODO

- Run `chromamancer apply-quick kitty -t themes/izar/theme.jsonc` (or `--stdout`) and diff against the checked-in snippet.
- Optional: Nix `runCommand` / HM `home.file` to write `~/.config/kitty/chromamancer-colors.conf` and `include` it from `kitty.conf`.

## Pitfalls

- Stylix: `targets.kitty` may be disabled intentionally so Kitty colors are not overwritten by another theme system.
- Kitty expects `#rrggbb` or `#rrggbbaa`; theme tokens use `#RRGGBBAA` — emitter normalizes.
