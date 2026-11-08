# Bundled presets

Each `<key>.toml` here is a bundled preset in the specification's preset
format (S11): its `[preset]` table, its `[declare]` tables, its bases
(`[inputs.…]`, `[colors.light.…]` and `[colors.dark.…]`, `[tokens.…]`,
`[widgets.…]`, `[components.…]`), its `[[rules]]`, and its `[pending]`
table, which lists the paths its facts leave pending, each with its register
id (S17). `Theme::from_preset(PresetId, Scheme)` loads one, and
`Theme::set_preset` replaces the preset of a theme.

## Generated from the facts

A preset is not written by hand. `scripts/generate_preset.py` writes it from
its platform's verified facts (`docs/facts/<platform>/`), each value with a
comment naming the fact it comes from. What no fact states is written by hand,
between `# hand-written: begin <part>` and `# hand-written: end <part>`: the
`[preset]` table's `generation`, the `[declare]` keyed namespaces, and the
bases the `read` facts give (a key's reference to a reader input; S11, S17).
The file's fifth line, `# generated lines: sha256 <hex>`, is the digest of
every line outside those blocks, so `--write` refuses a file whose generated
lines were edited by hand.

```sh
python3 scripts/generate_preset.py <key> --write   # regenerate, the hand-written blocks carried over
python3 scripts/generate_preset.py <key> --check   # whether --write would change the file
```

`scripts/generate_preset.py`'s docstring states which facts a preset takes,
what it leaves out and why, and how the hand-written blocks are carried.

## The checks

- `generate_preset.py <key> --check`, for each preset here, in
  `scripts/check_release.sh` (`--quick` and the data documents section): the
  file is what `--write` writes from the facts, byte for byte.
- The facts gate (`native-theme/tests/facts_gate.rs`, S17): each preset
  resolved with no reader, every widget's state-model combinations and every
  token, colour-role and input fact compared with the fact that applies.
- `every_bundled_preset_validates` (S19.1's build test): each preset reads,
  states its own key as its `id`, and validates.
- `scripts/check_preset_citations.py`: a comment that cites a specification
  section in place of a fact (`font_dpi = 72  # S9.4`) holds only for the key,
  value and presets the section defines.

## The list

`mod.rs` declares `PresetId`, one variant per file here: its key (S11's `id`,
`preset:<key>` and `for_preset.<key>`) and its file, which the crate embeds.
A preset is added with its facts: the platform's facts in
`docs/facts/<platform>/`, `generate_preset.py <key> --write`, the hand-written
blocks, the `PresetId` entry, and the checks above green.

| File | Preset |
|------|--------|
| `adwaita.toml` | GNOME's libadwaita |
| `macos-tahoe.toml` | Apple's macOS 26 Tahoe |
| `macos-monterey.toml` | Apple's macOS 12 Monterey to macOS 15, its facts read on macOS 15 |
| `material-3.toml` | Google's Material 3 |
| `ios-26.toml` | Apple's iOS 26 |
| `ios-13.toml` | Apple's iOS 13 to iOS 18, its facts read on iOS 18 |
| `kde-breeze.toml` | KDE's Breeze, Plasma 6 |
| `windows-11.toml` | Microsoft's Windows 11 (WinUI 3) |

## Naming a platform preset

One platform preset per visual generation. Its key is the vendor's own name
for the generation's first OS version: Apple's codename (`macos-tahoe`),
Microsoft's version number (`windows-11`), Material's design version
(`material-3`). The style presets keep their style names, `kde-breeze` and
`adwaita`, since the KDE and GNOME readers read the installed style, and the
style, not the desktop's version, is the generation. The `[preset]` table's
`generation` (`libadwaita-1.10`, `macos-26`) is written by hand, citing its
source's entry in `docs/facts/sources.toml`.
