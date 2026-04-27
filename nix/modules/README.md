# Nix modules (deferred)

**Bootstrap phase:** chromamancer is meant to run as a **standalone CLI**, not as a flake-integrated Home Manager / NixOS module from this repository.

When we add first-party integration later, this directory would hold:

- `home-manager.nix` — user-session wiring (generated fragments, optional options).
- `nixos.nix` — optional system-level hooks.

The parent `flake.nix` keeps commented placeholders for future `homeManagerModules` / `nixosModules` exports. Until then, **`devShells` only**.
