#!/usr/bin/env python3
"""Assemble pre-captured PNG frames into an animated theme-switching GIF.

scripts/generate_gifs_theme_switching.sh captures each showcase's frames and
calls this script once per connector, with that connector's output path:

    python3 scripts/generate_gifs_assemble.py FRAME_DIR OUTPUT_GIF

Each frame keeps its captured size; frames of differing sizes are refused.
Exit 0 when the GIF is written, 1 when it is not (no frames, or frames of
differing sizes).

Requirements:
  - Python 3 + Pillow
"""

import argparse
import glob
import os
import sys

from PIL import Image


def assemble_theme_switching_gif(frame_dir, output_path, hold_ms=2000):
    """Assemble pre-captured PNG frames into an animated theme-switching GIF.

    Reads all frame-*.png files from the frame directory (sorted), keeps
    each at its captured size, the size of the screenshots beside it, so
    the README shows the window as large as it is (a reader's browser
    scales both to its column alike), converts to RGB with white
    background, and saves as a looping GIF. Frames of differing sizes are
    refused rather than scaled to one another.
    """
    frame_paths = sorted(glob.glob(os.path.join(frame_dir, "frame-*.png")))
    if not frame_paths:
        print(f"  ERROR: No frame-*.png files found in {frame_dir}")
        return False

    print(f"  Found {len(frame_paths)} frames in {frame_dir}")

    gif_frames = []
    for path in frame_paths:
        img = Image.open(path).convert("RGBA")
        if gif_frames and img.size != gif_frames[0].size:
            print(f"  ERROR: {os.path.basename(path)} is {img.size[0]}x{img.size[1]}, "
                  f"the first frame {gif_frames[0].size[0]}x{gif_frames[0].size[1]}")
            return False
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
        description="Assemble a theme-switching GIF from PNG frames."
    )
    parser.add_argument("frame_dir", help="The directory holding the frame-*.png files")
    parser.add_argument("output", help="The GIF to write (each connector's own path)")
    args = parser.parse_args()

    print("Assembling theme-switching GIF...")
    if assemble_theme_switching_gif(args.frame_dir, args.output):
        print("Done.")
        sys.exit(0)
    print("FAILED.")
    sys.exit(1)


if __name__ == "__main__":
    main()
