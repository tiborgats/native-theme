#!/usr/bin/env python3
"""Generate looping GIF animations from bundled spinner icons.

Produces GIF files in native-theme/docs/assets/ for genuine bundled spinners:
  - material   (progress_activity.svg, 24 rotation frames, 42ms/frame)
  - lucide     (loader.svg, 24 rotation frames, 42ms/frame)

Each GIF shows the spinner centered inside a styled card background.

Also supports --theme-switching mode: assembles pre-captured PNG frames
into an animated theme-switching GIF. Callers provide the exact output
path (per-connector), so the default is nominal only.

Requirements:
  - ImageMagick 7 (magick command) for SVG rasterization
  - Python 3 + Pillow for card compositing and GIF assembly
"""

import argparse
import glob
import os
import re
import subprocess
import sys
import tempfile

from PIL import Image, ImageDraw, ImageFont

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
ICONS_DIR = os.path.join(PROJECT_ROOT, "native-theme", "icons")

# Spinner configuration
SPINNERS = {
    "material": {
        "source": "rotation",
        "svg": os.path.join(ICONS_DIR, "material", "progress_activity.svg"),
        "frame_count": 24,
        "duration_ms": 42,
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


def generate_rotated_frames(svg_path, frame_count, tmpdir, name):
    """Generate rotated SVG frame files for a spinner icon.

    Reads the SVG, extracts its viewBox to find the rotation center,
    then wraps the content in a <g transform="rotate(...)"> element
    to produce each frame.
    """
    with open(svg_path) as f:
        svg_content = f.read()

    # Extract the SVG tag attributes and inner content
    svg_match = re.search(r"<svg([^>]*)>(.*?)</svg>", svg_content, re.DOTALL)
    if not svg_match:
        print(f"  ERROR: Could not parse {name} SVG")
        return []

    svg_attrs = svg_match.group(1)
    inner_content = svg_match.group(2)

    # Extract viewBox to compute rotation center
    vb_match = re.search(r'viewBox="([^"]*)"', svg_attrs)
    if vb_match:
        parts = vb_match.group(1).split()
        vb_x, vb_y, vb_w, vb_h = float(parts[0]), float(parts[1]), float(parts[2]), float(parts[3])
        cx = vb_x + vb_w / 2
        cy = vb_y + vb_h / 2
    else:
        cx, cy = 12, 12  # fallback for 24x24 icons

    frame_paths = []
    for i in range(frame_count):
        angle = i * (360.0 / frame_count)
        rotated_svg = (
            f'<svg{svg_attrs}>\n'
            f'  <g transform="rotate({angle:.1f} {cx:.1f} {cy:.1f})">'
            f"{inner_content}"
            "  </g>\n"
            "</svg>\n"
        )
        frame_path = os.path.join(tmpdir, f"{name}_frame_{i:02d}.svg")
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

    if config["source"] == "rotation":
        # Generate rotated frames from the icon SVG
        svg_frames = generate_rotated_frames(
            config["svg"], config["frame_count"], tmpdir, spinner_name
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


def assemble_theme_switching_gif(frame_dir, output_path, width=600, hold_ms=2000):
    """Assemble pre-captured PNG frames into an animated theme-switching GIF.

    Reads all frame-*.png files from the frame directory (sorted),
    resizes each to the target width maintaining aspect ratio,
    converts to RGB with white background, and saves as a looping GIF.
    """
    frame_paths = sorted(glob.glob(os.path.join(frame_dir, "frame-*.png")))
    if not frame_paths:
        print(f"  ERROR: No frame-*.png files found in {frame_dir}")
        return False

    print(f"  Found {len(frame_paths)} frames in {frame_dir}")

    gif_frames = []
    for path in frame_paths:
        img = Image.open(path).convert("RGBA")
        # Resize to target width, maintaining aspect ratio
        ratio = width / img.width
        new_size = (width, int(img.height * ratio))
        img = img.resize(new_size, Image.Resampling.LANCZOS)
        # Convert to RGB with white background (no GIF transparency,
        # per Phase 36-02 decision)
        rgb_frame = Image.new("RGB", img.size, (255, 255, 255))
        rgb_frame.paste(img, mask=img.split()[3])
        gif_frames.append(rgb_frame)
        print(f"    {os.path.basename(path)}: {img.size[0]}x{img.size[1]}")

    if not gif_frames:
        print("  ERROR: No frames processed")
        return False

    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    gif_frames[0].save(
        output_path,
        save_all=True,
        append_images=gif_frames[1:],
        duration=hold_ms,
        loop=0,
        disposal=2,
    )

    file_size = os.path.getsize(output_path)
    print(f"  -> {output_path} ({file_size:,} bytes, "
          f"{len(gif_frames)} frames, {hold_ms}ms/frame)")
    return True


def main():
    parser = argparse.ArgumentParser(
        description="Generate looping GIF animations from SVG spinner frames."
    )
    parser.add_argument(
        "--output-dir",
        default=os.path.join(PROJECT_ROOT, "native-theme", "docs", "assets"),
        help="Output directory for spinner GIF files (default: native-theme/docs/assets/)",
    )
    parser.add_argument(
        "--theme-switching",
        metavar="FRAME_DIR",
        help="Assemble a theme-switching GIF from PNG frames in the given directory",
    )
    parser.add_argument(
        "--theme-switching-output",
        default=os.path.join(
            PROJECT_ROOT, "connectors", "native-theme-iced", "docs", "assets", "theme-switching.gif"
        ),
        help=(
            "Output path for the theme-switching GIF. Callers should always "
            "supply this explicitly per connector; the default is nominal."
        ),
    )
    args = parser.parse_args()

    # Theme-switching mode: assemble frames into GIF and exit
    if args.theme_switching:
        print("Assembling theme-switching GIF...")
        if assemble_theme_switching_gif(
            args.theme_switching, args.theme_switching_output
        ):
            print("Done.")
            sys.exit(0)
        else:
            print("FAILED.")
            sys.exit(1)

    # Default mode: generate spinner GIFs
    output_dir = args.output_dir
    os.makedirs(output_dir, exist_ok=True)

    print("Generating spinner GIFs...")
    print(f"  Output: {output_dir}")
    print()

    success_count = 0
    fail_count = 0

    with tempfile.TemporaryDirectory(prefix="spinner_gifs_") as tmpdir:
        for name in ["material", "lucide"]:
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
    for name in ["material", "lucide"]:
        path = os.path.join(output_dir, f"spinner-{name}.gif")
        if os.path.exists(path):
            size = os.path.getsize(path)
            print(f"  {path} ({size:,} bytes)")


if __name__ == "__main__":
    main()
