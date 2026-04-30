# Firefox target

**Status:** **Documented** — real-world theming is done today via **Home Manager** (`programs.firefox` + `userChrome.css`) and optional **Stylix** / Base16 in the user’s NixOS config. There is **no** `targets/firefox/mapping.jsonc` or CLI **`apply-quick firefox`** in this repo yet; this file defines the **contract** so a future adapter can emit the same artifacts from **`themes/<id>/theme.jsonc`** (v3 shims).

## Why Firefox is its own target

- **GTK / Stylix** (`stylix.targets.gtk`, Base16 → `gtk.css`) colors many GTK apps, but on **Wayland** Firefox often **does not use GTK for the tab strip and toolbar** (Proton chrome). Changing Stylix alone may change **little or nothing** visible in tabs.
- **`ui.systemUsesDarkTheme`** (e.g. via NixOS `programs.firefox.preferences`) forces **Gecko’s built‑in dark UI**; tab surfaces can then follow **internal** greys (e.g. `#222d32`) instead of your palette and **override** the look you get from GTK.
- **Stylix `targets.firefox`** (Home Manager profile + `stylix.targets.firefox.profileNames`) is optional and injects profile-level theming; it is separate from “System theme follows GTK”.

For **Izar**-aligned chrome without waiting on GTK, **`userChrome.css`** on an HM-managed profile is the reliable lever.

## Contract (future `mapping.jsonc` v2)

When an adapter exists, expect:

- **`metadata`:** `{ "schema_version": "2", "target_id": "firefox" }` (after `firefox` is added to [`target-mapping-v2.schema.json`](../../specs/schemas/target-mapping-v2.schema.json) and a [`mapping.jsonc`](mapping.jsonc) lands here).
- **`shim_to_native`:** maps **[shims](../../specs/shim-colors.md)** → logical **userChrome / CSS variable** slots (or generated **`user.js`** prefs), not GTK keys. Native “keys” might be dotted ids like `navigator-toolbox.background` or `tab.selected.background` — **TBD** when the adapter is designed.

### Suggested shim → chrome roles (Izar-oriented)

These mirror what a hand-written `userChrome` does today for **Izar**; exact selectors may drift with Firefox ESR/release.

| Shim (theme v3) | Chrome role | Example hex (`themes/izar`) |
|-----------------|-------------|-----------------------------|
| `base` / `window` | Tab strip / toolbox behind tabs | depth `#0B0A1C`, void `#010212` |
| `dark` / `plum` | Selected tab background | plum `#302947` |
| `chrome_focus` | Selected tab accent line | teal `#6ABAB5` |
| `text` / `foreground` | Tab labels, toolbar text | lavender `#D7CADC` |

## What works in practice today (outside this repo)

Reference layout (NixOS + Home Manager), not vendored here:

- **Base16 for Stylix GTK** — e.g. `themes/izar-base16.yaml` + `stylix.base16Scheme` so GTK-adjacent UI matches Izar.
- **HM `programs.firefox.profiles.<name>`** — `settings."toolkit.legacyUserProfileCustomizations.stylesheets" = true` and **`userChrome`** with explicit backgrounds/labels for `#navigator-toolbox`, `.tabbrowser-tab`, `.tab-label`, etc.
- **Profiles** — Firefox may use **`~/.mozilla/firefox`** or **`~/.config/mozilla/firefox`**; HM and Stylix targets must agree on **which profile** is default (`about:support` → Profile folder).

## Automation TODO

- Add **`targets/firefox/mapping.jsonc`** (v2) + register **`firefox`** in schema / [`logic-registry.md`](../../specs/logic-registry.md).
- Implement **`chromamancer apply-quick firefox`** (and/or **`apply-nix`**) emitting `userChrome.css` + optional `userContent.css` fragments into a chosen profile path or stdout.
- Optional: integrate **Stylix `targets.firefox`** with the same profile names as HM.

## Pitfalls

- **Selector churn** — Firefox UI updates can break `userChrome` selectors; pin Firefox channel or re-test after upgrades.
- **`!important`** — Often required to beat Proton defaults; keep rules minimal to reduce maintenance.
- **Do not** rely on **`ui.systemUsesDarkTheme`** alone if you want tabs to match a **custom** palette — it optimizes for Mozilla’s dark chrome, not your tokens.

## See also

- [`../README.md`](../README.md) — target index and mapping rules.
- [`../../themes/izar/theme.jsonc`](../../themes/izar/theme.jsonc) — Izar token source.
- [Stylix installation](https://nix-community.github.io/stylix/installation.html) / [Firefox module](https://nix-community.github.io/stylix/options/modules/firefox.html) — Stylix + HM Firefox.
