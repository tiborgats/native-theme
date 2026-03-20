#!/usr/bin/env python3
"""Generate looping GIF animations from bundled SVG spinner frames.

Produces GIF files in docs/assets/ for all 3 bundled spinner types:
  - material   (12 frames, 83ms/frame, ~1s cycle)
  - adwaita    (20 frames, 60ms/frame, 1.2s cycle)
  - lucide     (24 generated rotation frames, 42ms/frame, ~1s cycle)

Each GIF shows the spinner centered inside a styled card background
with a label, simulating the showcase context.

Requirements:
  - ImageMagick 7 (magick command) for SVG rasterization
  - Python 3 + Pillow for card compositing and GIF assembly
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile

from PIL import Image, ImageDraw, ImageFont

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
ANIMATIONS_DIR = os.path.join(PROJECT_ROOT, "native-theme", "animations")
ICONS_DIR = os.path.join(PROJECT_ROOT, "native-theme", "icons")

# Spinner configuration
SPINNERS = {
    "material": {
        "source": "frames",
        "dir": os.path.join(ANIMATIONS_DIR, "material"),
        "frame_count": 12,
        "duration_ms": 83,
    },
    "adwaita": {
        "source": "frames",
        "dir": os.path.join(ANIMATIONS_DIR, "adwaita"),
        "frame_count": 20,
        "duration_ms": 60,
    },
    "lucide": {
        "source": "rotation",
        "svg": os.path.join(ICONS_DIR, "lucide", "loader.svg"),
        "frame_count": 24,
        "duration_ms": 42,
    },
}

# Card styling
CARD_WIDTH = 200
CARD_HEIGHT = 120
CARD_BG_COLOR = (245, 245, 245)  # #f5f5f5
CARD_BORDER_COLOR = (224, 224, 224)  # #e0e0e0
CARD_CORNER_RADIUS = 8
SPINNER_RENDER_SIZE = 48
LABEL_COLOR = (100, 100, 100)  # #646464

# Display names for spinner labels
SPINNER_LABELS = {
    "material": "Material",
    "adwaita": "Adwaita",
    "lucide": "Lucide",
}


def replace_current_color(svg_content, color="#333333"):
    """Replace currentColor with a visible color for standalone rendering."""
    return svg_content.replace("currentColor", color)


def rasterize_svg(svg_path, output_png, size, tmpdir=None):
    """Render an SVG file to a PNG using ImageMagick.

    Replaces currentColor with #333333 before rendering.
    """
    with open(svg_path) as f:
        svg_content = f.read()

    svg_content = replace_current_color(svg_content)

    # Write modified SVG to temp file
    if tmpdir is None:
        tmpdir = tempfile.gettempdir()
    tmp_svg = os.path.join(tmpdir, "tmp_render.svg")
    with open(tmp_svg, "w") as f:
        f.write(svg_content)

    cmd = [
        "magick",
        "-background", "none",
        "-density", "192",
        tmp_svg,
        "-resize", f"{size}x{size}",
        output_png,
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  WARNING: magick failed for {svg_path}: {result.stderr.strip()}")
        return False
    return True


def rasterize_svg_content(svg_content, output_png, size, tmpdir):
    """Render SVG content string to a PNG using ImageMagick."""
    svg_content = replace_current_color(svg_content)

    tmp_svg = os.path.join(tmpdir, "tmp_render.svg")
    with open(tmp_svg, "w") as f:
        f.write(svg_content)

    cmd = [
        "magick",
        "-background", "none",
        "-density", "192",
        tmp_svg,
        "-resize", f"{size}x{size}",
        output_png,
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  WARNING: magick failed: {result.stderr.strip()}")
        return False
    return True


def generate_lucide_rotated_frames(svg_path, frame_count, tmpdir):
    """Generate rotated SVG frame files for the lucide loader spinner.

    The lucide loader is a single SVG that should be rotated through 360
    degrees. We wrap its content in a <g transform="rotate(...)"> element
    to produce each frame.
    """
    with open(svg_path) as f:
        svg_content = f.read()

    # Extract the inner content (paths) between the <svg ...> and </svg> tags
    # The lucide SVG has attributes on the <svg> tag we need to preserve
    inner_match = re.search(r"<svg[^>]*>(.*?)</svg>", svg_content, re.DOTALL)
    if not inner_match:
        print("  ERROR: Could not parse lucide loader SVG")
        return []

    inner_content = inner_match.group(1)

    frame_paths = []
    for i in range(frame_count):
        angle = i * (360.0 / frame_count)
        rotated_svg = (
            '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"'
            ' stroke="currentColor" stroke-width="2"'
            ' stroke-linecap="round" stroke-linejoin="round">\n'
            f'  <g transform="rotate({angle:.1f} 12 12)">'
            f"{inner_content}"
            "  </g>\n"
            "</svg>\n"
        )
        frame_path = os.path.join(tmpdir, f"lucide_frame_{i:02d}.svg")
        with open(frame_path, "w") as f:
            f.write(rotated_svg)
        frame_paths.append(frame_path)

    return frame_paths


def create_card_background(label):
    """Create a styled card background image with a label."""
    card = Image.new("RGBA", (CARD_WIDTH, CARD_HEIGHT), (0, 0, 0, 0))
    draw = ImageDraw.Draw(card)

    # Draw rounded rectangle background
    draw.rounded_rectangle(
        [(0, 0), (CARD_WIDTH - 1, CARD_HEIGHT - 1)],
        radius=CARD_CORNER_RADIUS,
        fill=CARD_BG_COLOR + (255,),
        outline=CARD_BORDER_COLOR + (255,),
        width=1,
    )

    # Draw label text at the bottom
    try:
        font = ImageFont.truetype("/usr/share/fonts/noto/NotoSans-Regular.ttf", 13)
    except (OSError, IOError):
        try:
            font = ImageFont.truetype("/usr/share/fonts/TTF/DejaVuSans.ttf", 13)
        except (OSError, IOError):
            font = ImageFont.load_default()

    bbox = draw.textbbox((0, 0), label, font=font)
    text_width = bbox[2] - bbox[0]
    text_x = (CARD_WIDTH - text_width) // 2
    text_y = CARD_HEIGHT - 28
    draw.text((text_x, text_y), label, fill=LABEL_COLOR + (255,), font=font)

    return card


def composite_frame(card_bg, spinner_frame, spinner_size):
    """Composite a spinner frame onto the center of the card background."""
    composite = card_bg.copy()

    # Resize spinner frame to desired display size
    spinner = spinner_frame.resize(
        (spinner_size, spinner_size), Image.Resampling.LANCZOS
    )

    # Center the spinner on the card (shifted up slightly for label room)
    x = (CARD_WIDTH - spinner_size) // 2
    y = (CARD_HEIGHT - spinner_size) // 2 - 10

    composite.paste(spinner, (x, y), spinner)
    return composite


def generate_spinner_gif(spinner_name, config, output_dir, tmpdir):
    """Generate a GIF for a single spinner type."""
    label = SPINNER_LABELS.get(spinner_name, spinner_name)
    print(f"  Generating {spinner_name} ({config['frame_count']} frames, "
          f"{config['duration_ms']}ms/frame)...")

    # Create card background
    card_bg = create_card_background(label)

    # Get frame PNG paths
    frame_pngs = []

    if config["source"] == "frames":
        # Render existing SVG frame files to PNG
        for i in range(config["frame_count"]):
            svg_path = os.path.join(config["dir"], f"frame_{i:02d}.svg")
            if not os.path.exists(svg_path):
                print(f"    ERROR: Missing frame {svg_path}")
                return False

            png_path = os.path.join(tmpdir, f"{spinner_name}_frame_{i:02d}.png")
            if not rasterize_svg(svg_path, png_path, SPINNER_RENDER_SIZE, tmpdir):
                return False
            frame_pngs.append(png_path)

    elif config["source"] == "rotation":
        # Generate rotated frames for lucide spinner
        svg_frames = generate_lucide_rotated_frames(
            config["svg"], config["frame_count"], tmpdir
        )
        if not svg_frames:
            return False

        for i, svg_path in enumerate(svg_frames):
            png_path = os.path.join(tmpdir, f"{spinner_name}_frame_{i:02d}.png")
            if not rasterize_svg(svg_path, png_path, SPINNER_RENDER_SIZE, tmpdir):
                return False
            frame_pngs.append(png_path)

    # Composite frames onto card background
    composited_frames = []
    for png_path in frame_pngs:
        spinner_img = Image.open(png_path).convert("RGBA")
        frame = composite_frame(card_bg, spinner_img, SPINNER_RENDER_SIZE)
        # Convert to palette mode for GIF (with transparency handling)
        composited_frames.append(frame)

    if not composited_frames:
        print(f"    ERROR: No frames generated for {spinner_name}")
        return False

    # Convert RGBA frames to RGB with white background for GIF
    # (GIF transparency is limited; solid background is cleaner)
    gif_frames = []
    for frame in composited_frames:
        rgb_frame = Image.new("RGB", frame.size, (255, 255, 255))
        rgb_frame.paste(frame, mask=frame.split()[3])
        gif_frames.append(rgb_frame)

    # Assemble GIF
    output_path = os.path.join(output_dir, f"spinner-{spinner_name}.gif")
    gif_frames[0].save(
        output_path,
        save_all=True,
        append_images=gif_frames[1:],
        duration=config["duration_ms"],
        loop=0,
        disposal=2,
    )

    file_size = os.path.getsize(output_path)
    cycle_ms = config["frame_count"] * config["duration_ms"]
    print(f"    -> {output_path} ({file_size:,} bytes, {cycle_ms}ms cycle)")
    return True


def main():
    parser = argparse.ArgumentParser(
        description="Generate looping GIF animations from SVG spinner frames."
    )
    parser.add_argument(
        "--output-dir",
        default=os.path.join(PROJECT_ROOT, "docs", "assets"),
        help="Output directory for GIF files (default: docs/assets/)",
    )
    args = parser.parse_args()

    output_dir = args.output_dir
    os.makedirs(output_dir, exist_ok=True)

    print("Generating spinner GIFs...")
    print(f"  Output: {output_dir}")
    print()

    success_count = 0
    fail_count = 0

    with tempfile.TemporaryDirectory(prefix="spinner_gifs_") as tmpdir:
        for name in ["material", "adwaita", "lucide"]:
            config = SPINNERS[name]
            if generate_spinner_gif(name, config, output_dir, tmpdir):
                success_count += 1
            else:
                fail_count += 1

    print()
    print(f"Summary: {success_count} GIFs generated, {fail_count} failures")

    if fail_count > 0:
        sys.exit(1)

    # List all generated files
    print()
    print("Generated files:")
    for name in ["material", "adwaita", "lucide"]:
        path = os.path.join(output_dir, f"spinner-{name}.gif")
        if os.path.exists(path):
            size = os.path.getsize(path)
            print(f"  {path} ({size:,} bytes)")


if __name__ == "__main__":
    main()
