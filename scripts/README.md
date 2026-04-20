# Scripts

Visual asset generation and release automation for native-theme.

All scripts output to `docs/assets/` and are run from the project root.

## generate_assets.sh

Master orchestration script. Runs all four asset generators in sequence:
spinner GIFs, iced screenshots, gpui screenshots, and theme-switching GIFs.

```sh
./scripts/generate_assets.sh
```

## generate_gifs.py

Generates looping spinner GIF animations from the bundled Material and Lucide
SVG icons. Each GIF shows the spinner centered on a styled card background
(24 rotation frames, 42ms/frame).

Also supports `--theme-switching` mode to assemble pre-captured PNG frames
into an animated theme-switching GIF for the README hero section.

Requires: Python 3, Pillow, ImageMagick 7

```sh
# Spinner GIFs
python3 scripts/generate_gifs.py

# Theme-switching GIF from captured frames
python3 scripts/generate_gifs.py --theme-switching /path/to/frames \
    --theme-switching-output docs/assets/iced-theme-switching.gif
```

## generate_screenshots.sh

Captures iced showcase screenshots on Linux (KDE Wayland) using spectacle.
Launches the showcase with each theme/variant/icon-set combination, waits for
it to render, then captures the active window.

On macOS/Windows, use the showcase's built-in `--screenshot` flag instead.

Requires: spectacle (KDE)

```sh
./scripts/generate_screenshots.sh
```

## generate_gpui_screenshots.sh

Same as `generate_screenshots.sh` but for the gpui showcase. Includes
`--icon-theme` for freedesktop themes that need an explicit icon theme name.

Requires: spectacle (KDE)

```sh
./scripts/generate_gpui_screenshots.sh
```

## generate_theme_switching_gif.sh

Captures 4 theme presets from both iced and gpui showcases, then assembles
each set into a looping theme-switching GIF via `generate_gifs.py`.

Produces: `iced-theme-switching.gif` and `gpui-theme-switching.gif`.

Requires: spectacle (KDE), Python 3, Pillow

```sh
./scripts/generate_theme_switching_gif.sh
```

## render-diagrams.sh

Regenerates SVG diagrams from Mermaid `.mmd` sources in `docs/assets/`.
Uses `mermaid-cli` via `npx` — no global install required.

Run after editing any `.mmd` file. The generated `.svg` is checked into
git so contributors don't need Node installed to view diagrams.

Requires: Node.js ≥ 18, network access (first run downloads `mmdc` on demand)

```sh
./scripts/render-diagrams.sh
```

## pre-release.sh

Full pre-release asset pipeline. Triggers the CI screenshots workflow for
macOS/Windows, generates all local Linux assets while CI runs, then downloads
the CI artifacts into `docs/assets/`.

Requires: gh CLI (authenticated), spectacle, Python 3, Pillow, ImageMagick 7

```sh
./scripts/pre-release.sh
```
