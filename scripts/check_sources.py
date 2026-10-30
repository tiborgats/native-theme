#!/usr/bin/env python3
"""The source cache's list, docs/facts/sources.toml (plan D1, specification S17): well formed, and complete.

Form: every field is a string, `generations`, `sparse` and `sources` lists of strings (a field of another type is
reported and left out of the other checks); every [[source]] has the fields of its kind and no others; ids are unique
and `<name>@<version>` (for a commit pin, `tag = "commit"`, the version is the commit's first eight hex digits); SHAs
are 40 hex (SHA-256s 64); every generation is a preset id of the plan; one pin per name and generation. Every [[site]]
names its method, and a "repository" site the [[source]] ids of its repositories. Every [[not_pinned]] gives its
reason. A project is named by one list only.

Completeness: every project scripts/citations.py finds cited in the core, the catalogue, the traces, the gap maps and
docs/platform-facts.md (citations.SOURCES_SCOPE; quotes of other documents' cells left out) is the `project` of a
[[source]], a [[site]] or a [[not_pinned]]. A citation that names no project is not checked (the re-pin lists those).
Offline: reads the documents and this repository's file list only, not the source cache.

    python3 scripts/check_sources.py [sources.toml]
    python3 scripts/check_sources.py --self-test
"""
import re
import sys
import tomllib
from pathlib import Path

import citations

PRESETS = ("adwaita", "kde-breeze", "macos-monterey", "macos-tahoe", "ios-13", "ios-26", "windows-11", "material-3")
ID_RX = re.compile(r"^([A-Za-z0-9][A-Za-z0-9._+-]*)@([A-Za-z0-9][A-Za-z0-9._+-]*)$")
SHA_RX = re.compile(r"^[0-9a-f]{40}$")
SHA256_RX = re.compile(r"^[0-9a-f]{64}$")
DATE_RX = re.compile(r"^\d{4}-\d\d-\d\d$")
COMMON = {"id", "kind", "release", "generations", "researched", "proof"}
KINDS = {
    # kind: (required beyond COMMON, optional)
    "git": ({"repository", "tag", "commit"}, {"project", "site", "tag_object", "arch", "sparse", "note"}),
    # An sdk entry is an Xcode on a runner image (its header citations checked by os-probe.yml's macOS job) or a
    # system image another harness measures (the Android emulator's); SDK_SHAPES holds each one's fields.
    "sdk": (set(), {"runner", "image_version", "macos", "xcode", "xcode_build", "readme", "readme_commit",
                    "ios_simulator", "package", "revision", "archive", "sha256", "fingerprint", "note"}),
    "distribution": ({"url", "lastupdate", "extra_db_sha256", "core_db_sha256"}, {"note"}),
}
SDK_SHAPES = {
    "xcode": {"runner", "image_version", "macos", "xcode", "xcode_build", "readme", "readme_commit"},
    "system image": {"package", "revision", "archive", "sha256", "fingerprint"},
}
SITE_FIELDS = ({"project", "url", "method"}, {"sources", "note"})
NOT_PINNED_FIELDS = ({"name", "reason"}, {"project"})
LIST_FIELDS = {"generations", "sparse", "sources"}   # lists of strings; every other field is a string


class Entry(dict):
    """A table of sources.toml without its fields of the wrong type (`bad`, reported by entries())."""
    bad = frozenset()


def typed(key, value):
    if key in LIST_FIELDS:
        return isinstance(value, list) and all(isinstance(x, str) for x in value)
    return isinstance(value, str)


def entries(data, table, out):
    """The [[table]] entries, each field of its type: a field that is not is reported, and left out."""
    raw = data.get(table, [])
    if not isinstance(raw, list):
        out.append(f"`{table}` is not an array of [[{table}]] tables")
        return []
    got = []
    for i, e in enumerate(raw):
        if not isinstance(e, dict):
            out.append(f"{table} #{i + 1} is not a table")
            continue
        ok = Entry((k, v) for k, v in e.items() if typed(k, v))
        ok.bad = frozenset(e.keys() - ok.keys())
        label = ok.get("id") or ok.get("project") or ok.get("name") or f"#{i + 1}"
        out += [f"{table} {label}: {k} is not {'a list of strings' if k in LIST_FIELDS else 'a string'}"
                for k in sorted(ok.bad)]
        got.append(ok)
    return got


def fields_problems(where, entry, required, optional):
    bad = getattr(entry, "bad", frozenset())
    out = [f"{where}: missing field `{f}`" for f in sorted(required - entry.keys() - bad)]
    out += [f"{where}: unknown field `{f}`" for f in sorted(entry.keys() - required - optional)]
    return out


def source_problems(e):
    sid = e.get("id", "?")
    where = f"source {sid}"
    kind = e.get("kind")
    if kind not in KINDS:
        return [f"{where}: kind {kind!r} is not one of {', '.join(KINDS)}"]
    required, optional = KINDS[kind]
    out = fields_problems(where, e, COMMON | required, optional)
    m = ID_RX.match(sid)
    if not m:
        out.append(f"{where}: id is not `<name>@<version>`")
    gens = e.get("generations")
    if not isinstance(gens, list) or not gens:
        out.append(f"{where}: generations is not a non-empty list")
    else:
        out += [f"{where}: generation {g!r} is not a preset id" for g in gens if g not in PRESETS]
        if len(set(gens)) != len(gens):
            out.append(f"{where}: a generation is listed twice")
    if "researched" in e and not DATE_RX.match(str(e["researched"])):
        out.append(f"{where}: researched {e['researched']!r} is not a YYYY-MM-DD date")
    if kind == "git":
        for f in ("commit", "tag_object"):
            if f in e and not SHA_RX.match(str(e[f])):
                out.append(f"{where}: {f} {e[f]!r} is not a 40-hex SHA")
        if not str(e.get("repository", "")).startswith("https://"):
            out.append(f"{where}: repository is not an https URL")
        if e.get("tag") == "commit":
            if "tag_object" in e:
                out.append(f"{where}: a commit pin has a tag_object")
            if m and SHA_RX.match(str(e.get("commit", ""))) and m.group(2) != e["commit"][:8]:
                out.append(f"{where}: a commit pin's version is not its commit's first eight hex digits")
        for p in e.get("sparse", []):
            if not p or p.startswith("/") or ".." in p.split("/"):
                out.append(f"{where}: sparse path {p!r} is not a relative folder")
        if "project" not in e and "site" not in e and "note" not in e:
            out.append(f"{where}: neither a project, a site nor a note saying why it is pinned")
    if kind == "sdk":
        shape = "xcode" if "xcode" in e else "system image"
        for f in sorted(SDK_SHAPES[shape] - set(e)):
            out.append(f"{where}: the {shape} sdk entry lacks {f}")
        if shape == "xcode":
            if not SHA_RX.match(str(e.get("readme_commit", ""))):
                out.append(f"{where}: readme_commit is not a 40-hex SHA")
            if m and m.group(2) != str(e.get("xcode")):
                out.append(f"{where}: the id's version is not the Xcode version")
        else:
            if not SHA256_RX.match(str(e.get("sha256", ""))):
                out.append(f"{where}: sha256 is not a 64-hex SHA-256")
            if m and "revision" in e and not m.group(2).endswith(f"-r{e['revision']}"):
                out.append(f"{where}: a system image's id does not end in -r<revision>")
    if kind == "distribution":
        for f in ("extra_db_sha256", "core_db_sha256"):
            if f in e and not SHA256_RX.match(str(e[f])):
                out.append(f"{where}: {f} is not a 64-hex SHA-256")
    return out


def check_form(data):
    """Problems in sources.toml's form, and the projects it covers (project -> which list names it)."""
    out = [f"unknown table `{k}`" for k in data if k not in ("source", "site", "not_pinned")]
    sources, sites, not_pinned = (entries(data, t, out) for t in ("source", "site", "not_pinned"))
    ids = set()
    pairs = set()
    covered = {}

    def cover(project, by):
        if project in covered:
            out.append(f"project {project!r} named by {covered[project]} and by {by}")
        else:
            covered[project] = by

    for e in sources:
        out += source_problems(e)
        sid = e.get("id", "?")
        if sid in ids:
            out.append(f"source {sid}: id not unique")
        ids.add(sid)
        name = sid.split("@")[0]
        for g in e.get("generations", []) if isinstance(e.get("generations"), list) else []:
            if (name, g) in pairs:
                out.append(f"source {sid}: a second pin of {name} for {g}")
            pairs.add((name, g))
    projects = {}
    for e in sources:
        if "project" in e:
            projects.setdefault(e["project"], []).append(e.get("id", "?"))
    for p, which in projects.items():
        cover(p, "source " + ", ".join(which))
    seen_sites = set()
    for s in sites:
        where = f"site {s.get('project', '?')}"
        out += fields_problems(where, s, *SITE_FIELDS)
        if s.get("project") in seen_sites:
            out.append(f"{where}: listed twice")
        seen_sites.add(s.get("project"))
        if s.get("method") not in ("repository", "snapshot"):
            out.append(f"{where}: method {s.get('method')!r} is not repository or snapshot")
        if s.get("method") == "repository":
            if not s.get("sources"):
                out.append(f"{where}: a repository site names no sources")
            out += [f"{where}: source {x} is not a [[source]] id" for x in s.get("sources", []) if x not in ids]
        elif "sources" in s:
            out.append(f"{where}: a snapshot site names sources")
        if "project" in s:
            cover(s["project"], "a site")
    names = set()
    for n in not_pinned:
        where = f"not_pinned {n.get('name', '?')}"
        out += fields_problems(where, n, *NOT_PINNED_FIELDS)
        if n.get("name") in names:
            out.append(f"{where}: listed twice")
        names.add(n.get("name"))
        if not str(n.get("reason", "")).strip():
            out.append(f"{where}: empty reason")
        if "project" in n:
            cover(n["project"], "not_pinned")
    return out, covered


def cited_projects(records):
    """project -> (citations, the first `doc:line`) of every non-quote record naming a project."""
    out = {}
    for c in records:
        if c.quote:
            continue
        for p in citations.projects(c):
            n, first = out.get(p, (0, f"{c.doc}:{c.line}"))
            out[p] = (n + 1, first)
    return out


def check(data, cited):
    out, covered = check_form(data)
    for p, (n, first) in sorted(cited.items()):
        if p not in covered:
            out.append(f"cited project {p!r} ({n} citations, first {first}) is neither pinned, a listed site nor "
                       f"not_pinned")
    return out


SELF_TEST = '''
[[source]]
id = "breeze@6.8.2"
kind = "git"
project = "KDE/breeze"
repository = "https://invent.kde.org/plasma/breeze.git"
tag = "v6.8.2"
commit = "c97883a8fe41bce774835c25d7f7e321c71fa618"
tag_object = "b10d17a54bdf1425e4fcf10deac30dbde8229eb3"
release = "Plasma 6.8.2"
generations = ["kde-breeze"]
researched = "2026-10-29"
proof = "git ls-remote"

[[source]]
id = "docs@f54166ed"
kind = "git"
site = "learn.microsoft.com"
repository = "https://github.com/MicrosoftDocs/windows-dev-docs.git"
tag = "commit"
commit = "f54166edfc21db4151077f5ef7e3f93b776843cb"
release = "Learn"
generations = ["windows-11"]
researched = "2026-10-29"
proof = "git ls-remote"
sparse = ["hub/apps/design"]

[[source]]
id = "xcode@26.6"
kind = "sdk"
runner = "macos-26"
image_version = "20261013.0424.1"
macos = "26.6.2 (25G83)"
xcode = "26.6"
xcode_build = "17F113"
readme = "images/macos/macos-26-arm64-Readme.md"
readme_commit = "d490982abb0217949a4cca2950c8d9903137280a"
release = "macos-26"
generations = ["macos-tahoe", "ios-26"]
researched = "2026-10-29"
proof = "the README"

[[site]]
project = "learn.microsoft.com"
url = "https://learn.microsoft.com/"
method = "repository"
sources = ["docs@f54166ed"]

[[site]]
project = "developer.apple.com HIG"
url = "https://developer.apple.com/design/human-interface-guidelines/"
method = "snapshot"

[[not_pinned]]
name = "native-theme"
project = "native-theme (own)"
reason = "own files"
'''

SELF_TEST_CITED = {"KDE/breeze": (3, "gap-map-kde.md:9"), "learn.microsoft.com": (1, "gap-map-windows.md:35"),
                   "native-theme (own)": (2, "spec.md:4")}


def self_test() -> int:
    failures = []
    good = SELF_TEST

    def expect(label, text, want, cited=None):
        got = check(tomllib.loads(text), SELF_TEST_CITED if cited is None else cited)
        if got != want:
            failures.append(f"{label}: got {got}, want {want}")

    def edit(text, old, new):
        if old not in text:
            failures.append(f"fixture: {old!r} not found")
        return text.replace(old, new, 1)

    expect("good", good, [])
    expect("duplicate id", edit(good, 'id = "docs@f54166ed"', 'id = "breeze@6.8.2"'),
           ["source breeze@6.8.2: a commit pin's version is not its commit's first eight hex digits",
            "source breeze@6.8.2: id not unique",
            "site learn.microsoft.com: source docs@f54166ed is not a [[source]] id"])
    expect("id form", edit(good, 'id = "breeze@6.8.2"', 'id = "breeze 6.8.2"'),
           ["source breeze 6.8.2: id is not `<name>@<version>`"])
    expect("short SHA", edit(good, 'commit = "c97883a8fe41bce774835c25d7f7e321c71fa618"', 'commit = "c97883a8"'),
           ["source breeze@6.8.2: commit 'c97883a8' is not a 40-hex SHA"])
    expect("unknown generation", edit(good, 'generations = ["kde-breeze"]', 'generations = ["kde"]'),
           ["source breeze@6.8.2: generation 'kde' is not a preset id"])
    expect("missing field", edit(good, 'researched = "2026-10-29"\nproof = "git ls-remote"\n\n[[source]]\nid = "docs',
                                 'proof = "git ls-remote"\n\n[[source]]\nid = "docs'),
           ["source breeze@6.8.2: missing field `researched`"])
    expect("unknown field", edit(good, 'tag = "v6.8.2"', 'tag = "v6.8.2"\ntga = "v6.8.2"'),
           ["source breeze@6.8.2: unknown field `tga`"])
    expect("commit pin's version", edit(good, 'id = "docs@f54166ed"', 'id = "docs@main"'),
           ["source docs@main: a commit pin's version is not its commit's first eight hex digits",
            "site learn.microsoft.com: source docs@f54166ed is not a [[source]] id"])
    expect("second pin per generation", edit(good, 'id = "docs@f54166ed"\nkind = "git"\nsite = "learn.microsoft.com"',
                                              'id = "breeze@f54166ed"\nkind = "git"\nsite = "learn.microsoft.com"').replace(
        'generations = ["windows-11"]', 'generations = ["kde-breeze"]'),
           ["source breeze@f54166ed: a second pin of breeze for kde-breeze",
            "site learn.microsoft.com: source docs@f54166ed is not a [[source]] id"])
    image = good + """
[[source]]
id = "android-system-image@37.1-r9"
kind = "sdk"
package = "system-images;android-37.1;google_apis_ps16k;x86_64"
revision = "9"
archive = "sys-img/google_apis/x86_64-ps16k-37.1_r09.zip"
sha256 = "11b9682183827379e8d415db880a30b9ec84d4032650e08e6149b69b30d545a0"
fingerprint = "google/sdk_gphone16k_x86_64/emu64xa16k:17/CP31.260623.012/16064790:userdebug/dev-keys"
release = "Android 17"
generations = ["material-3"]
researched = "2026-10-30"
proof = "spike A"
"""
    expect("system image sdk", image, [])
    expect("system image sha256", edit(image, 'sha256 = "11b96821', 'sha256 = "11b9682'),
           ["source android-system-image@37.1-r9: sha256 is not a 64-hex SHA-256"])
    expect("system image revision", edit(image, '37.1-r9"', '37.1-r8"'),
           ["source android-system-image@37.1-r8: a system image's id does not end in -r<revision>"])
    expect("system image fields", edit(image, 'fingerprint = "google', 'note = "google'),
           ["source android-system-image@37.1-r9: the system image sdk entry lacks fingerprint"])
    expect("sdk version", edit(good, 'id = "xcode@26.6"', 'id = "xcode@26.5"'),
           ["source xcode@26.5: the id's version is not the Xcode version"])
    expect("bad method", edit(good, 'method = "snapshot"', 'method = "archive"'),
           ["site developer.apple.com HIG: method 'archive' is not repository or snapshot"])
    expect("no reason", edit(good, 'reason = "own files"', 'reason = " "'), ["not_pinned native-theme: empty reason"])
    expect("pinned and not pinned", edit(good, 'project = "native-theme (own)"', 'project = "KDE/breeze"'),
           ["project 'KDE/breeze' named by source breeze@6.8.2 and by not_pinned",
            "cited project 'native-theme (own)' (2 citations, first spec.md:4) is neither pinned, a listed site nor "
            "not_pinned"])
    expect("a field of another type", edit(good, 'tag = "v6.8.2"', 'tag = 6'),
           ["source breeze@6.8.2: tag is not a string"])
    expect("generations not a list", edit(good, 'generations = ["windows-11"]', 'generations = "windows-11"'),
           ["source docs@f54166ed: generations is not a list of strings",
            "source docs@f54166ed: generations is not a non-empty list"])
    expect("a site not a table", 'site = 3\n' + good.split("[[site]]")[0],
           ["`site` is not an array of [[site]] tables", "cited project 'learn.microsoft.com' (1 citations, first "
            "gap-map-windows.md:35) is neither pinned, a listed site nor not_pinned", "cited project 'native-theme "
            "(own)' (2 citations, first spec.md:4) is neither pinned, a listed site nor not_pinned"])
    expect("cited, in no list", good, ["cited project 'GNOME/gtk' (5 citations, first gap-map-gnome.md:14) is neither "
                                       "pinned, a listed site nor not_pinned"],
           dict(SELF_TEST_CITED, **{"GNOME/gtk": (5, "gap-map-gnome.md:14")}))
    recs = citations.scan("`A/w/_buttons.scss:328`; [x](https://example.com/a)", "gap-map-gnome.md",
                          citations.Index())
    got = cited_projects(recs)
    if got != {"GNOME/libadwaita": (1, "gap-map-gnome.md:1"), "example.com": (1, "gap-map-gnome.md:1")}:
        failures.append(f"cited_projects: got {got}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv):
    if argv[1:] == ["--self-test"]:
        return self_test()
    path = Path(argv[1] if len(argv) > 1 else citations.SOURCES_TOML)
    try:
        data = tomllib.loads(path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as e:
        print(f"{path}: {e}", file=sys.stderr)
        return 2
    records = citations.scan_documents(citations.SOURCES_SCOPE)
    cited = cited_projects(records)
    problems = check(data, cited)
    unnamed = sum(1 for c in records if not c.quote and not citations.projects(c))
    print(f"# {len(data.get('source', []))} sources, {len(data.get('site', []))} sites, "
          f"{len(data.get('not_pinned', []))} not pinned")
    print(f"# {len(records)} citations in {len(citations.SOURCES_SCOPE)} documents, {len(cited)} projects cited")
    print(f"# {unnamed} citations name no project (not checked here)")
    for p in problems:
        print(p)
    print(f"# {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
