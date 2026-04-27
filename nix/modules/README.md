# Nix modules (deferred)

**Bootstrap phase:** chromamancer is meant to run as a **standalone CLI**, not as a flake-integrated Home Manager / NixOS module from this repository.

When we add first-party integration later, this directory would hold:

- `home-manager.nix` — user-session wiring (generated fragments, optional options).
- `nixos.nix` — optional system-level hooks.

**Nix integration (deferred for first-party modules):** see **Reference Nix** in [`specs/SPEC.md`](../specs/SPEC.md). This repo does not ship HM/NixOS modules in the bootstrap phase.
