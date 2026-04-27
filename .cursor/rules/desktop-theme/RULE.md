---
description: "Desktop theming suite — Nix, Hyprland, spec-driven schemes"
alwaysApply: true
---

## chromamancer (desktop-theme)

This project is a **spec-driven** theming suite for Linux desktops (Hyprland, Kitty, GTK/Qt, Quickshell, Albert, etc.).

### Priorities

1. **`specs/`** — treat schema changes as API changes; version them.
2. **`schemes/`** — data-only packs; no secrets; paths sensible for Nix store packaging.
3. **`nix/`** — declarative application via Home Manager / NixOS should consume the **same** artifacts as the CLI.
4. **`crates/cli/`** — fast iteration and future apply/watch workflows.

### Conventions

- Prefer **semantic tokens** (`bg0`, `accent`, …) over raw hex in target configs; map tokens in one place.
- When adding a target, document it in `specs/SPEC.md` and extend schemas as needed.
- Hyprland and Quickshell configs should stay **readable**; generate from scheme data rather than hand-duplicating colors.

### Nix

- Use the **flake** under `nix/` for dev shells; extend with `homeManagerModules` / `nixosModules` when ready.
- Pin `nixpkgs` deliberately; document breaking changes when bumping inputs.
