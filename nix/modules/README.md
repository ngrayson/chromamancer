# Nix modules (stubs)

Place **Home Manager** and **NixOS** module entrypoints here when you wire declarative theming.

- `home-manager.nix` — user session: Kitty, Hyprland, Quickshell, Albert, GTK/Qt theming via HM options.
- `nixos.nix` — optional system-level configuration if any targets require it.

The flake in the parent directory includes commented placeholders for exporting these as `homeManagerModules` / `nixosModules`.
