#!/usr/bin/env python3
"""Generate SVG spinner frame files for native-theme bundled animations.

Produces two sets of SVG frames under native-theme/animations/:
  - material/  (12 frames) -- circular arc with varying dash length
  - adwaita/   (20 frames) -- overlapping arcs with sinusoidal breathing

All frames use viewBox="0 0 24 24" and xmlns="http://www.w3.org/2000/svg".
"""

import math
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
ANIMATIONS_DIR = os.path.join(PROJECT_ROOT, "native-theme", "animations")

SVG_OPEN = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none">'
SVG_CLOSE = "</svg>"


def write_svg(subdir, frame_index, content):
    """Write an SVG frame file."""
    dirpath = os.path.join(ANIMATIONS_DIR, subdir)
    os.makedirs(dirpath, exist_ok=True)
    filepath = os.path.join(dirpath, f"frame_{frame_index:02d}.svg")
    with open(filepath, "w", newline="\n") as f:
        f.write(content)


def generate_material():
    """Generate 12 Material Design circular arc spinner frames.

    Uses a <circle> with stroke-dasharray to create a growing/shrinking arc
    that rotates 30 degrees per frame.
    """
    r = 9.5
    circumference = 2 * math.pi * r  # ~59.69

    for i in range(12):
        # Arc extent varies via sine wave: 27.5 +/- 12.5
        visible = 27.5 + 12.5 * math.sin(2 * math.pi * i / 12)
        gap = circumference - visible
        angle = i * 30  # 30 degrees per frame

        content = (
            f"{SVG_OPEN}\n"
            f'  <circle cx="12" cy="12" r="9.5" '
            f'stroke="currentColor" stroke-width="2.5" '
            f'stroke-linecap="round" fill="none" '
            f'stroke-dasharray="{visible:.2f} {gap:.2f}" '
            f'transform="rotate({angle} 12 12)"/>\n'
            f"{SVG_CLOSE}\n"
        )
        write_svg("material", i, content)

    print(f"  material: 12 frames")


def generate_adwaita():
    """Generate 20 GNOME Adwaita-style spinner frames.

    Single arc with dynamic start/end angles, 1200ms cycle (60ms per frame).
    Based on AdwSpinnerPaintable: phases of idle, extend, contract, overlap
    with sinusoidal easing.
    """
    r = 9.5
    cx, cy = 12.0, 12.0

    # Phase durations as fractions of the cycle (summing to 1.0)
    # idle: 0.9*PI extent, extend: grow to 1.1*PI, contract: shrink to 0.7*PI, overlap
    # Simplified: arc extent oscillates via sine between min and max
    min_extent = 0.7 * math.pi   # ~126 degrees
    max_extent = 1.35 * math.pi  # ~243 degrees

    for i in range(20):
        # Base rotation: 18 degrees per frame (360/20)
        base_angle = i * 18.0

        # Sinusoidal breathing on arc extent
        t = i / 20.0
        # ease-in-out-sine: eased = (1 - cos(t * 2*PI)) / 2
        # This creates a smooth oscillation
        eased = (1 - math.cos(t * 2 * math.pi)) / 2
        arc_extent_rad = min_extent + (max_extent - min_extent) * eased
        arc_extent_deg = math.degrees(arc_extent_rad)

        start_deg = base_angle
        end_deg = base_angle + arc_extent_deg

        start_rad = math.radians(start_deg)
        end_rad = math.radians(end_deg)

        x1 = cx + r * math.cos(start_rad)
        y1 = cy + r * math.sin(start_rad)
        x2 = cx + r * math.cos(end_rad)
        y2 = cy + r * math.sin(end_rad)

        large_arc = 1 if arc_extent_deg > 180 else 0

        content = (
            f"{SVG_OPEN}\n"
            f'  <path d="M {x1:.2f} {y1:.2f} A 9.5 9.5 0 {large_arc} 1 {x2:.2f} {y2:.2f}" '
            f'stroke="currentColor" stroke-width="2.5" '
            f'stroke-linecap="round" fill="none"/>\n'
            f"{SVG_CLOSE}\n"
        )
        write_svg("adwaita", i, content)

    print(f"  adwaita: 20 frames")


def verify():
    """Verify all generated frames have correct viewBox and xmlns."""
    errors = []
    total = 0
    for subdir, expected in [("material", 12), ("adwaita", 20)]:
        dirpath = os.path.join(ANIMATIONS_DIR, subdir)
        files = sorted(f for f in os.listdir(dirpath) if f.endswith(".svg"))
        if len(files) != expected:
            errors.append(f"{subdir}: expected {expected} files, found {len(files)}")
        for fname in files:
            total += 1
            fpath = os.path.join(dirpath, fname)
            with open(fpath) as f:
                content = f.read()
            if 'viewBox="0 0 24 24"' not in content:
                errors.append(f"{subdir}/{fname}: missing viewBox")
            if 'xmlns="http://www.w3.org/2000/svg"' not in content:
                errors.append(f"{subdir}/{fname}: missing xmlns")

    if errors:
        print("VERIFICATION FAILED:")
        for e in errors:
            print(f"  - {e}")
        return False
    else:
        print(f"VERIFICATION PASSED: {total} SVG files, all with correct viewBox and xmlns")
        return True


if __name__ == "__main__":
    print("Generating spinner frames...")
    generate_material()
    generate_adwaita()
    print()
    verify()
