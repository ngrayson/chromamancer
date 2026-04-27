# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/`. Version them (`scheme-v1`, later `scheme-v2`) before adding generators or Nix glue.
2. **Schemes are data** under `schemes/<name>/`. They must validate against the active schema (tooling TBD).
3. **Targets** (Kitty, GTK, Qt, Quickshell, Albert, Hyprland decorations, etc.) consume a **canonical token set** defined in the schema README and schema definitions — avoid one-off color names per app.
4. **Iteration**: the Rust CLI under `crates/cli/` will eventually apply schemes quickly; **Nix / Home Manager** applies the same artifacts declaratively once a look is finalized.

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

- `specs/schemas/scheme-v1.schema.json` — v1 draft JSON Schema (stub; evolve collaboratively).
- `schemes/README.md` — layout for scheme packs.
- `ARCHITECTURE.md` — repository layout and data flow.
