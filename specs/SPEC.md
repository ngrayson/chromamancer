# Specification index — chromamancer

Spec-driven desktop theming: live apply, Nix/HM, scheme packs, multi-target (Kitty, GTK, Qt, Quickshell, Albert, Hyprland)

## How we work (spec-driven)

1. **Contracts live in** `specs/schemas/`. Version them (`scheme-v1`, later `scheme-v2`) before adding generators or Nix glue.
2. **Schemes are data** under `schemes/<name>/`. They must validate against the active schema (tooling TBD).
3. **`scheme.json` is portable look data** — Base16 `tokens`, `fonts`, `assets`. **Default mapping** from our names to each app’s config (and transforms like derived borders) lives in **target adapters** inside chromamancer; you add targets one adapter at a time.
4. **Iteration vs system of record:** see **Apply model** below — CLI for fast targets during dev; when you adopt Nix for the same paths, **NixOS / Home Manager activation** replaces generated outputs.
5. **Bootstrap delivery (current phase):** chromamancer ships as a **standalone CLI**—a normal program you run (`chromamancer …`). It is **not** gated on **flakes** or **Home Manager**: there is no required first-party `homeManagerModules` / `nixosModules` output in this phase. Optional **devShell** in `nix/flake.nix` is only for building/hacking. **Later phase:** first-party Nix modules or flake installables if we choose to add them.

## Colors (v1): Base16 canonical names

**Why Base16:** Sixteen stable keys match terminal/editor ecosystems and give one row of swatches for generators to map onto **Qt `QPalette` roles**, **Kvantum**-style widgets, GTK, and Wayland compositor colors.

**Encoding:** every `tokens.*` value is **`#RRGGBBAA`** (no `#RRGGBB` shorthand in v1).

**Keys:** exactly `base00` … `base0F`, all required. Meanings follow the [Base16 styling guide](https://github.com/chriskempson/base16/blob/main/styling.md). The table below adds a **Qt / UI-oriented hint** so implementers know *typical* mapping targets (not exhaustive—Kvantum SVG themes may derive extra stops from these).

| Key | Base16 role (short) | Typical Qt / Kvantum / UI use |
|-----|---------------------|-------------------------------|
| `base00` | default background | `QPalette::Window`, main Kvantum window/base fill |
| `base01` | lighter background | `AlternateBase`, status bars, inactive tabs |
| `base02` | selection background | `Highlight` background-tones, list selection plane |
| `base03` | subtle / comments | borders, disabled, placeholder tone |
| `base04` | dark foreground | secondary `WindowText`, dim labels |
| `base05` | default foreground | `QPalette::Text`, primary content |
| `base06` | light foreground | emphasized labels, bright `WindowText` |
| `base07` | light background | popovers, tooltips base, elevated panels |
| `base08` | red / diff delete | destructive, error accents |
| `base09` | orange | constants / URIs emphasis |
| `base0A` | yellow | warning, search highlight, caution |
| `base0B` | green / diff add | success, positive indicators |
| `base0C` | cyan | links, info, quotes in syntax |
| `base0D` | blue | `QPalette::Highlight` / focus, links, “accent” in many themes |
| `base0E` | magenta | secondary accent, keyword emphasis |
| `base0F` | brown / deprecated | rare accents, legacy chrome |

**Swapping themes:** replace the pack (or `tokens` / `fonts` / `assets` data). Adapters stay the same so the same look applies everywhere.

## Target adapters (mapping + logic)

**Not in `scheme.json` by default:** the rules that say “`base0D` → Hyprland `general:col.active_border`” (and any **logic**—blending alpha, rounding, template snippets) live in **chromamancer**, one **adapter per target** (Rust first; Nix can call the CLI, reuse shared tables, or embed parallel logic documented to stay in sync).

- **Version adapters** with the tool (e.g. `hyprland_v1`) so old schemes keep working when mapping tables change.
- **Add targets incrementally** — shipping a new adapter enables that target for **all** scheme packs without editing each pack.
- **Optional `target_overrides`** in `scheme.json` — per-target JSON objects merged **on top of** adapter output when a particular palette needs an exception (rare); shape is adapter-defined. **Merge semantics** are fixed (see below).

### `target_overrides` merge semantics (v1)

**Merge order:** for each target id `T` (e.g. `hyprland`), start from the adapter’s **default output object** (from `tokens`, `fonts`, `assets`), then merge **`target_overrides[T]`** on top. **Override wins** when both sides define the same key.

**Objects:** if both values are JSON **objects**, merge **recursively** with the same rule at each level.

**Arrays, strings, numbers, booleans, null:** if the **override** value for a key is **not** a JSON object, it **replaces** the adapter’s value for that key in full. In particular, **arrays are replaced whole**—no element-by-element merge in v1 unless a **specific adapter** documents otherwise.

**Unknown keys:** keys that appear only in `target_overrides` are merged in like any others. There is **no** global step that rejects unknown override keys. **Each adapter** decides at **emit time** whether to forward them, strip them, or **fail** (e.g. if the target format is strict). Adapters should document behavior; prefer **explicit errors** over silent data loss for strict downstreams.

Default Base16 → Qt/Kvantum-style roles remains documented in the **Colors** table above; each adapter’s concrete key list will be spelled out in adapter-specific docs or tests as we implement.

## Apply model: fast iteration vs Nix

**Standalone bootstrap:** if you **only** run the chromamancer CLI, it is the **sole** writer to the paths you configure—nothing automatically overwrites its output until **you** introduce another mechanism (Nix, another tool, etc.).

**When you use Nix for the same paths:** the rules below apply.

**Two classes of target (per adapter):**

1. **Fast-iterative** — chromamancer **CLI** can regenerate outputs under the live config tree and, where the app supports it, **reload** so you can tune a look quickly. Typical for file-based configs that accept included fragments (e.g. Hyprland, Kitty); exact list is per-adapter documentation.
2. **Rebuild-only** — outputs are only sensible to apply as part of **NixOS / Home Manager** activation (no hot reload, or generation is tied to store paths / system state). Iteration loop is **edit scheme → rebuild**, not `chromamancer apply`.

**Authoritative Nix (when you opt in):** if **`nixos-rebuild`** / **`home-manager switch`** (or equivalent) **installs** the same generated paths chromamancer uses, that activation **regenerates** those files (via **your** `runCommand`, **future** first-party module, etc.) and **overwrites** anything the CLI wrote there. Treat overlapping **CLI apply** as **dev-time** in that case; persistent state belongs in **your** Nix-expressed scheme inputs and derivations.

**Implication:** after a rebuild that manages those paths, on-disk theme files should match **what Nix installed**, not stale CLI tweaks—unless paths are intentionally disjoint (CLI → dev copy, Nix → production copy).

## CLI: apply modes (bootstrap)

The tool exposes **two** apply entry points. They share the **same** generation pipeline (scheme + adapters + merge); they differ in **where outputs are written** and **who** eventually owns the live target files.

| Command | Writes to | Live target files |
|---------|-----------|-------------------|
| **`chromamancer apply-quick`** | **Directly** to the **actual paths** each running target reads (e.g. the real Hyprland / Kitty fragment or config paths under **`$XDG_CONFIG_HOME`**—exact paths **per adapter**). | Updated **immediately** by chromamancer; reload/restart as needed. |
| **`chromamancer apply-nix`** | **Only** into your **Nix configuration tree** (paths your flake / Home Manager / NixOS config imports—e.g. `generated/` in your dotfiles). **Not** to live `~/.config` as the first hop. | Written when you run **`nixos-rebuild switch`** / **`home-manager switch`** (or equivalent): Nix **activates** and installs from the store (or equivalent) **into** those target paths. |

For **`apply-nix`**, the output location inside the Nix tree is **user-configured** (e.g. **`--out`** and/or **`CHROMAMANCER_NIX_OUT`**—exact names TBD) so it matches how your modules reference generated files.

**Workflow:** use **`apply-quick`** while iterating locally; use **`apply-nix`** so committed Nix config + **`switch`** is what lands on disk for production. After a successful **`switch`**, live files should match what Nix installed—not stale **`apply-quick`** edits unless your Nix expressions point at the same paths (usually avoid mixing without intent).

Shared flags (conceptual): scheme path / pack id, target selection, dry-run—specified when implementing.

## Supported targets (adapter roadmap)

| Target        | Role of adapter | Typical iteration class |
|---------------|-----------------|-------------------------|
| Kitty         | Map Base16 → terminal colors + `fonts.mono.family`; optional overrides | usually **fast** |
| GTK           | Palette / theme fragments from Base16 + `fonts.ui.family` | often **rebuild-only** |
| Qt / Kvantum  | `QPalette` / Kvantum-related output + `fonts.ui.family` | often **rebuild-only** |
| Quickshell    | Bar / lock theming from tokens + assets | **TBD** per setup |
| Albert        | QSS / theme from palette | **TBD** |
| Hyprland      | `general:col.*`, decoration-related keys from mapping + logic | usually **fast** |

Exact **fast vs rebuild-only** is declared per adapter when implemented; the table is planning guidance only.

## Fonts (v1): global `fonts`

The scheme carries **which typefaces** belong to the look, not **how big** they are—font sizes stay in Nix/Home Manager options, per-target configs (e.g. Kitty), or personal prefs so DPI and ergonomics do not fight the palette.

**v1 requires both slots:** `fonts.ui` and `fonts.mono` are always present in a valid scheme (JSON Schema enforces this). Qt/GTK-side generators use `ui`; terminal-style targets use `mono`. If you want one typeface everywhere, set both `family` strings to the same value.

- **`fonts.ui.family`** — proportional UI face (GTK, Qt/Kvantum, shell UI text where applicable).
- **`fonts.mono.family`** — monospace face (terminal, code-ish UI).

Each `family` is a **Linux-usable font family string** (usually a Fontconfig family name).

## Scheme pack file (v1)

- **Path:** `schemes/<scheme-id>/scheme.json` at the pack root (fixed filename; no nested-only layout in v1).
- **Syntax:** **JSONC** — JSON plus line (`//`) and block (`/* … */`) comments. Plain **JSON** (no comments) is always valid.
- **Semantics:** After parsing comments away, the document must validate against `specs/schemas/scheme-v1.schema.json` (structure is still “JSON” for schema tooling).
- **Optional:** `target_overrides` — see **Target adapters** above.
- **Nix note:** `builtins.fromJSON` / `readFile`+`fromJSON` only accept strict JSON. Nix-side pipelines should either consume a **generated JSON** artifact (e.g. `chromamancer dump-json`, a flake `runCommand` with a JSONC parser) or keep a machine-produced `scheme.json` without comments for import.

## Assets

Schemes may reference **wallpapers**, **avatars**, and other images. Paths should be **relative to the scheme directory** or explicitly declared so Nix can copy them into the store.

## Security

Do **not** store secrets in scheme files. Treat schemes as **public** configuration and assets.

## See also

- `specs/schemas/scheme-v1.schema.json` — v1 JSON Schema (Base16 + `#RRGGBBAA` + fonts + optional `target_overrides`); instance documents are **JSONC** on disk (`scheme.json`).
- `schemes/README.md` — layout for scheme packs.
- `ARCHITECTURE.md` — repository layout and data flow.
