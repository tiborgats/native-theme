# iOS host, run 42275185017 (2026-11-06)

[The run](https://github.com/tiborgats/native-theme/actions/runs/42275185017): `os-probe.yml`'s ios-host job with
`ios_host_passes=measure` and `ios_host_measure=fonts`, the measurement campaign's IP-fonts pass, second run. The
harness is `tools/ios-host` on `v0.8.0-tools-ios-m2` at `bf33742a`; what the pass reads and draws is as in
[run 42269041636](../2026-11-06-run-42269041636/README.md) (its `fonts` part), with the numbers inside dictionaries and
arrays written in NSNumber's own decimal form: the variation axis identifiers are whole here (`wdth` 2003072104,
`opsz` 1869640570, `GRAD` 1196572996, `wght` 2003265652).

| Folder | Preset | Runner | Xcode, SDK | Simulator |
|---|---|---|---|---|
| `ios-host-macos-15/` | `ios-13` | macOS 15.8.2 (24H232) | Xcode 16.4 (16F6), iphonesimulator 18.5 (22F76) | iOS 18.6 (22G86), iPhone 16 Pro |
| `ios-host-macos-26/` | `ios-26` | macOS 26.6.2 (25G83) | Xcode 26.6 (17F113), iphonesimulator 26.5 (23F81a) | iOS 26.5 (23F77), iPhone 17 Pro |

Kept here, per image folder: `measure1/index.toml` (the environment, `[[font_style]]`, `[font_sizes]`, `[[font_role]]`,
`[[cascade]]`, `[[font_for_string]]`, a `[[scene_read]]` per scene, a `[[capture]]` per crop), `measure1/screen/…`,
`solved/measure1.toml` and `measure-differ-levels.txt`.

What the run shows: both runs finished `ok` on both images. On iOS 18 the two indexes are identical and the crops
differ by at most one level; on iOS 26 the crops differ by more only in the `button_titles` scene's glass and prominent glass crops (Liquid Glass
adapts to its backdrop) and those crops' statistics in the index.
