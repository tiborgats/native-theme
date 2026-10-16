# OS probe, run 38478334574 (2026-10-14)

What each GitHub runner image's OS reported to `tools/os-probe` (`native-theme-os-probe`),
dispatched by hand on `v0.7.0-rc1` at `5ce19fcd` through `.github/workflows/os-probe.yml`
(https://github.com/tiborgats/native-theme/actions/runs/38478334574), on 2026-10-14 at 00:31
CEST (2026-10-13 22:31 UTC). Kept here because GitHub deletes run artifacts after 90 days, and
`docs/platform-facts.md` cites them (§6 of the GPUI Kit 0.7.3 specification).

| Directory | OS reported | Runner image version |
|-----------|-------------|----------------------|
| `os-probe-macos-14` | macOS 14.8.9 (23J631) | 20260831.0302.1 |
| `os-probe-macos-15` | macOS 15.7.9 (24G830) | 20260907.0337.1 |
| `os-probe-macos-26` | macOS 26.6.2 (25G83) | 20260907.0351.1 |
| `os-probe-xcode-27` | macOS 27.0.1 (26A434), GitHub's preview image | 20261006.0244.1 |
| `os-probe-windows-2022` | Windows Server 2022 Datacenter 21H2, build 20348.5622 | 20261004.326.1 |
| `os-probe-windows-2025` | Windows Server 2025 Datacenter 24H2, build 26100.33438 | 20260925.250.1 |
| `os-probe-windows-11-arm` | Windows 11 25H2, build 26200.9457 (the registry's `ProductName` says "Windows 10 Enterprise", as it does on every Windows 11) | 20261004.176.1 |

The images are the ones the first run read
([`2026-10-11-run-38129819806/`](../2026-10-11-run-38129819806/)), and every value that run
reported is the same here (only the object addresses in the macOS pattern colours'
descriptions differ). This run adds two readings:

- **macOS:** `[macos.metrics.NSTableView.header]`, the header height of a new `NSTableView` with
  one column, for its default style (`automatic`, effective `inset`), `inset` and `fullWidth`,
  in `aqua` and `darkAqua`, each read with the table in no window and in an offscreen window
  (`probe.toml:503-588`). Every reading on all four images is 28.0 pt.
- **Windows:** `SPI_GETMOUSEHOVERTIME` under `[windows.system_parameters]`
  (`probe.toml:703`): 400 ms on all three images.

Each directory holds `probe.toml`; the macOS ones also hold `renders/aqua/` and
`renders/darkAqua/`, 17 controls at the regular size, each at 1× and 2×, drawn in a window
that is never shown (`cacheDisplayInRect:`). None is blank; each is byte-identical to the first
run's.
