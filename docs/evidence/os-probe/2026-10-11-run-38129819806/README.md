# OS probe, run 38129819806 (2026-10-11)

What each GitHub runner image's OS reported to `tools/os-probe` (`native-theme-os-probe`),
dispatched by hand on `v0.7.0-rc1` at `05f6c11d` through `.github/workflows/os-probe.yml`
(https://github.com/tiborgats/native-theme/actions/runs/38129819806). Kept here because
GitHub deletes run artifacts after 90 days, and `docs/platform-facts.md` cites them
(M2 Step 3 of `docs/todo_code-review-plan.md`).

| Directory | OS reported | Runner image version |
|-----------|-------------|----------------------|
| `os-probe-macos-14` | macOS 14.8.9 (23J631) | 20260831.0302.1 |
| `os-probe-macos-15` | macOS 15.7.9 (24G830) | 20260907.0337.1 |
| `os-probe-macos-26` | macOS 26.6.2 (25G83) | 20260907.0351.1 |
| `os-probe-xcode-27` | macOS 27.0.1 (26A434), GitHub's preview image | 20261006.0244.1 |
| `os-probe-windows-2022` | Windows Server 2022 Datacenter 21H2, build 20348 | |
| `os-probe-windows-2025` | Windows Server 2025 Datacenter 24H2, build 26100 | |
| `os-probe-windows-11-arm` | Windows 11 25H2, build 26200 (the registry's `ProductName` says "Windows 10 Enterprise", as it does on every Windows 11) | |

Each directory holds `probe.toml`; the macOS ones also hold `renders/aqua/` and
`renders/darkAqua/`, 17 controls at the regular size, each at 1× and 2×, drawn in a window
that is never shown (`cacheDisplayInRect:`). None is blank.
