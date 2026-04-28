# Albert target (Widgets Box Model)

**Status:** Validated **manually** (Izar palette + window keys written to an Albert theme `.ini`). **No** `apply-quick` / emitter in the CLI yet — mapping and theme resolution are spec-ready; automation still to be implemented.

## Contract

- **Mapping:** [`mapping.jsonc`](mapping.jsonc) — `shim_to_native` values are **`section.key`** strings (`palette.highlight`, `window.input_border_brush`, …). Arrays mean the **same** resolved shim color applies to every listed key (Albert needs many keys to share one accent).
- **Spec:** [`../../specs/shim-colors.md`](../../specs/shim-colors.md) (list / launcher shims); Albert INI layout is implied by the `palette.*` vs `window.*` prefixes.

**Defaults in mapping:**

- `apply_nix.themes_dir`: `.local/share/albert/widgetsboxmodel/themes`
- `apply_nix.theme_file_template`: `{theme_id}.ini`  
  (Adapter should expand `{theme_id}` from `metadata.name` in `theme.jsonc`.)

## What has worked in practice

- **Output path (typical):** `~/.local/share/albert/widgetsboxmodel/themes/<ThemeId>.ini` (capitalization may match launcher expectations).
- **Sections:** `[palette]` (base, text, highlight, highlight_text, …) and `[window]` (input_*, result_*, action_*).
- **Transparent borders:** `selection_border` → `transparent` in theme `shim_assign`; maps to the `window.*_selection_border_brush` keys.
- **Theme:** [`themes/izar/theme.jsonc`](../../themes/izar/theme.jsonc) assigns all shims referenced by this mapping (including `list_selection` vs `chrome_focus` split for row accent vs other chrome).

## Automation TODO

- Implement emitter: resolve v3 theme → walk `shim_to_native` (flatten arrays) → write INI hex (Albert format TBD in code; often `#AARRGGBB` or engine-specific — confirm against Albert docs / current hand file).
- Wire `CHROMAMANCER_TARGETS_DIR` + `apply_nix` defaults for install path.
- Optional: CI golden file from `themes/izar` + `targets/albert/mapping.jsonc`.

## Pitfalls

- **One shim → many INI keys:** arrays are required; do not collapse to a single key per shim.
- **`palette.highlight_text` vs Kvantum `highlight.text`:** Izar maps Albert `highlight_text` through **`list_selection_foreground`** (void on teal rows), not `chrome_focus_text` — keep mapping as-is when automating.
