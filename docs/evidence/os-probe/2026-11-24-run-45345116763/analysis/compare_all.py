"""Every field of every key of two controls.toml files compared: per key the fields that differ (old -> new)."""
import sys, tomllib
old = tomllib.load(open(sys.argv[1], 'rb')); new = tomllib.load(open(sys.argv[2], 'rb'))
for key in sorted(set(old) & set(new)):
    d = [(f, old[key].get(f), new[key].get(f)) for f in sorted(set(old[key]) | set(new[key])) if old[key].get(f) != new[key].get(f)]
    if d:
        print(key)
        for f, a, b in d:
            print(f"    {f}: {a} -> {b}")
