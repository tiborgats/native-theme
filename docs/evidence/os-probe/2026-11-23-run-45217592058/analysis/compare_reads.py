"""Compare two controls.toml files of measure_controls.py key by key.

For every key in both: drawn_box, fill, label, ink, edge_* and whether diff_from_normal is zero; prints the count of
keys that read the same, the keys found in one file only, and per differing key the fields that differ (old, new).

Usage: python3 -I compare_reads.py OLD.toml NEW.toml
"""
import sys
import tomllib

FIELDS = ["drawn_box", "fill", "label", "ink", "edge_top", "edge_bottom", "edge_left", "edge_right"]


def zero(entry):
    d = entry.get("diff_from_normal")
    return None if d is None else d.get("differing_pixels") == 0


def main():
    with open(sys.argv[1], "rb") as f:
        old = tomllib.load(f)
    with open(sys.argv[2], "rb") as f:
        new = tomllib.load(f)
    both = sorted(set(old) & set(new))
    differing = []
    for key in both:
        d = [(f, old[key].get(f), new[key].get(f)) for f in FIELDS if old[key].get(f) != new[key].get(f)]
        if zero(old[key]) != zero(new[key]):
            d.append(("diff_from_normal zero", zero(old[key]), zero(new[key])))
        if d:
            differing.append((key, d))
    print(f"keys in both: {len(both)}, the same: {len(both) - len(differing)}, differing: {len(differing)}")
    print("only in OLD:", sorted(set(old) - set(new)))
    print("only in NEW:", sorted(set(new) - set(old)))
    for key, d in differing:
        print(key)
        for field, a, b in d:
            print(f"    {field}: {a} -> {b}")


if __name__ == "__main__":
    main()
