#!/usr/bin/env python3
"""Generate SVG spinner frame files for native-theme bundled animations.

Produces four sets of SVG frames under native-theme/animations/:
  - material/  (12 frames) -- circular arc with varying dash length
  - macos/     (12 frames) -- radial spokes with rotating opacity
  - windows/   (60 frames) -- arc expansion/contraction
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


def generate_macos():
    """Generate 12 macOS-style radial spoke spinner frames.

    Each frame has 12 line elements (spokes) with opacity gradient.
    The leading spoke rotates one position per frame.
    """
    inner_r = 6.0
    outer_r = 9.5
    num_spokes = 12

    for frame_i in range(12):
        lines = []
        for spoke_j in range(num_spokes):
            theta = math.radians(spoke_j * 30)
            # Polar to cartesian: x = 12 + r*sin(theta), y = 12 - r*cos(theta)
            x1 = 12 + inner_r * math.sin(theta)
            y1 = 12 - inner_r * math.cos(theta)
            x2 = 12 + outer_r * math.sin(theta)
            y2 = 12 - outer_r * math.cos(theta)

            # Opacity: leading spoke (index == frame_i) is 1.0,
            # subsequent spokes fade. Distance is (spoke_j - frame_i) mod 12.
            dist = (spoke_j - frame_i) % 12
            opacity = max(0.15, 1.0 - dist * 0.077)

            lines.append(
                f'  <line x1="{x1:.2f}" y1="{y1:.2f}" '
                f'x2="{x2:.2f}" y2="{y2:.2f}" '
                f'stroke="currentColor" stroke-width="2" '
                f'stroke-linecap="round" opacity="{opacity:.2f}"/>'
            )

        content = f"{SVG_OPEN}\n" + "\n".join(lines) + f"\n{SVG_CLOSE}\n"
        write_svg("macos", frame_i, content)

    print(f"  macos: 12 frames")


def generate_windows():
    """Generate 60 Windows-style arc expansion/contraction spinner frames.

    Two-phase animation over 60 frames (2-second cycle at 33ms/frame):
    - Frames 0-29: arc grows from ~5 to ~180 degrees
    - Frames 30-59: arc shrinks from ~180 to ~5 degrees
    Total rotation per cycle: ~900 degrees (15 degrees per frame).
    """
    r = 9.5
    cx, cy = 12.0, 12.0

    for i in range(60):
        # Base rotation: 15 degrees per frame
        base_angle = i * 15.0

        # Arc extent with ease-in-out sine
        if i < 30:
            # Growing phase
            t = i / 29.0
            eased = (1 - math.cos(t * math.pi)) / 2
            arc_extent = 5 + (180 - 5) * eased
        else:
            # Shrinking phase
            t = (i - 30) / 29.0
            eased = (1 - math.cos(t * math.pi)) / 2
            arc_extent = 180 - (180 - 5) * eased

        start_deg = base_angle
        end_deg = base_angle + arc_extent

        start_rad = math.radians(start_deg)
        end_rad = math.radians(end_deg)

        x1 = cx + r * math.cos(start_rad)
        y1 = cy + r * math.sin(start_rad)
        x2 = cx + r * math.cos(end_rad)
        y2 = cy + r * math.sin(end_rad)

        large_arc = 1 if arc_extent > 180 else 0

        content = (
            f"{SVG_OPEN}\n"
            f'  <path d="M {x1:.2f} {y1:.2f} A 9.5 9.5 0 {large_arc} 1 {x2:.2f} {y2:.2f}" '
            f'stroke="currentColor" stroke-width="2.5" '
            f'stroke-linecap="round" fill="none"/>\n'
            f"{SVG_CLOSE}\n"
        )
        write_svg("windows", i, content)

    print(f"  windows: 60 frames")


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
    for subdir, expected in [("material", 12), ("macos", 12), ("windows", 60), ("adwaita", 20)]:
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
    generate_macos()
    generate_windows()
    generate_adwaita()
    print()
    verify()
