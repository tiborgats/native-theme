"""The macOS checked/pressed accent reads of run 45345116763 against the macos-tahoe preset's mixes: the checked check
box's fill, the slider's filled track and the source-list row, at rest and pressed, under the default (blue) and the
red accent (AppleAccentColor 0). Per read the solved layer's straight colour; the track at the middle of the filled
part (left of the knob, on the knob's middle row, in the normal and pressed layers of the slider's press). The test:
the preset mixes the accent A with W (black in light, white in dark) by 0.025 at rest and 0.12 pressed, so a rest
read R predicts the pressed P = W + (R - W) * 0.88 / 0.975 per channel; printed beside the measured P, with the
largest channel difference. Run from the repository root: python3 <run>/analysis/accent_mix.py <run>"""
import sys, tomllib, pathlib, importlib.util
run = pathlib.Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("mc", run / "analysis/measure_controls.py")
mc = importlib.util.module_from_spec(spec); spec.loader.exec_module(mc)

def hexrgba(s):
    return [int(s[i:i + 2], 16) for i in (1, 3, 5, 7)]

def predict(rest, w):
    return [round(w + (c - w) * 0.88 / 0.975) for c in rest[:3]]

for image in ("macos-15", "macos-26"):
    for folder, pdir, accent in ((f"os-probe-{image}-controls", "button-pass-controls", "blue"),
                                 (f"os-probe-{image}-controls-accent", "button-pass-controls-accent", "red")):
        root = run / folder / pdir
        probe = tomllib.loads((root / "probe.toml").read_text())
        c = probe["macos"]["button_pass"]["controls"]
        reads = tomllib.loads((run / folder / "controls.toml").read_text())
        t = c["targets"]["slider_knob"]
        vx, vy, vw, vh = t["view_in_window_from_top_left_xywh"]
        kx, ky, kw, kh = t["part_in_window_from_top_left_xywh"]
        tx, ty = round(vx + (kx - vx) / 2), round(ky + kh / 2)
        for app, w in (("aqua", 0), ("darkAqua", 255)):
            a = c[app]
            def layer_px(state):
                lay = state["layer"]
                layer = mc.Layer(root / lay["file"], tuple(lay["file_rect_px_xywh"][:2]))
                return layer.px(tx, ty)
            rows = []
            for name, rest, pressed in (
                ("checkbox_on fill", reads[f"{app}.normal.checkbox_on"]["fill"], reads[f"{app}.pressed.checkbox_on"]["fill"]),
                ("source_list_row fill", reads[f"{app}.normal.source_list_row"]["fill"], reads[f"{app}.pressed.source_list_row"]["fill"]),
                ("slider track", mc.hexa(layer_px(a["normal"])), mc.hexa(layer_px(a["pressed"]["slider_knob"]))),
            ):
                r, p = hexrgba(rest), hexrgba(pressed)
                pred = predict(r, w)
                d = max(abs(x - y) for x, y in zip(pred, p[:3]))
                print(f"{image} {accent:4} {app:8} {name:21} rest {rest} pressed {pressed} predicted #{''.join(f'{v:02x}' for v in pred)} max_delta {d}")

# The source-list row is not checked at rest (its rest read is the unselected row), so its pressed read is fitted
# instead: the accent A taken from the checked check box at rest (A = W + (R - W) / 0.975, the preset's 0.025 mix
# undone), then per channel the weight k with P = A + (W - A) k; one weight fits where the three agree.
print()
for image in ("macos-26",):
    for folder, accent in ((f"os-probe-{image}-controls", "blue"), (f"os-probe-{image}-controls-accent", "red")):
        reads = tomllib.loads((run / folder / "controls.toml").read_text())
        for app, w in (("aqua", 0), ("darkAqua", 255)):
            r = hexrgba(reads[f"{app}.normal.checkbox_on"]["fill"])
            p = hexrgba(reads[f"{app}.pressed.source_list_row"]["fill"])
            a = [w + (c - w) / 0.975 for c in r[:3]]
            ks = [round((pc - ac) / (w - ac), 3) if abs(w - ac) > 8 else None for pc, ac in zip(p[:3], a)]
            print(f"{image} {accent:4} {app:8} source_list_row pressed {reads[f'{app}.pressed.source_list_row']['fill']}"
                  f" accent #{''.join(f'{max(0, min(255, round(v))):02x}' for v in a)} k per channel {ks}")
