#!/usr/bin/env python3
"""Find the citations in the 0.8.0 planning documents and resolve each to a project and a ref.

A citation is any reference to a source the documents read: a Markdown link or a bare URL to a code host or a
documentation site, a `name@sha path:L`, an abbreviation the documents define (`st:5327`, `A/w/_buttons.scss:328`,
`kwin src/x.cpp:140`), a bare `file:L`, a `:L` continuation of the citation before it, a `platform-facts.md:LINE`, an
Apple documentation path (`uikit/x/y`) or a page named by its title (`Learn *Title*`, `HIG Alerts`). The syntaxes and
their families are those of the 0.8.0 source inventory (A: project and commit in the citation; B: an abbreviation the
gap maps define; C: carried from earlier in the cell or paragraph; D: bare, no project; E: a documentation page;
F: this repository's own files; G: toolkit sources).

The rules, in the order resolve_token applies them to a path token:

1. a ref glued to the file (`RenderThemeMac.mm@de320ba3`);
2. a `name@sha` or `Org/name @ sha` prefix right before the path (ALIAS maps the short names);
3. a word right before the path: an abbreviation (WORD_ABBR: `kwin`, `kwa`, `MUX`, `CTK`, `qtbase v6.10.1`, …), a
   project the KDE file defines a commit for elsewhere (DOCDEFS), or `gpui`/`iced`/`egui`, which name this
   repository's connector or showcase when an own file ends in the path and the toolkit otherwise;
4. a path-prefix abbreviation before an ellipsis (`ax/… material3/Menu.kt`), then as the path's start (PATH_ABBR,
   longest first; `qt/`'s ref depends on the seeds file's section heading);
5. a line abbreviation (LINE_ABBR: `st:5327`);
6. this repository's own files (a known top folder or file name; an `at <sha>` after it is its ref);
7. a bare path carried from the cell's or paragraph's last resolved citation, accepted where the path exists in that
   project's checkout (heuristic where there is no checkout);
8. in the Windows gap map, the document's base: MUX (`controls/dev/`) or CTK (`components/`), told apart by existence;
9. the file name looked up in the checkouts and the repository (heuristic, or ambiguous for several projects); a
   path into the 0.8.0 session's scratchpad checkouts (`src/breeze/…`) takes that checkout's commit (SCRATCHPAD);
   then a trailing `at <ref>` or `, vX.Y` sets the ref where the citation gave none.

An abbreviation's ref (rules 3-5, 8, and the projects of DOCDEFS) is the one its definition states where the
documents are read (scan_documents(), or definitions() handed to scan()): definitions() reads, at the definition's
lines (ABBR's "defined at"), the first SHA and tag after the token in its clause, else the last before it; a
definition naming no ref ("same commit") takes the one of the nearest definition of the same project above it. So a
re-pinned definition is read as it stands; the tables' refs serve only where no document is read.

A `:L` continuation inherits the file of the citation before it in text order (a link, a platform-facts reference or a
path). A table whose header's first column is "platform-facts line" or "Line(s)" (the gap maps) has platform-facts
line numbers in that column. Every table cell is a unit of its own; a prose paragraph is one unit. Records of the
seeds file's G36 section and its "Section 2: platform-facts.md unknown (❓) cells" quote other documents' cells, the
latter cut at a fixed width, so they are marked `quote` (their URLs may be truncated); the quoted documents hold the
cited cells.

The checkouts (an Index) decide existence; without them the rules that need one fall back as stated. Index.of_repo()
holds this repository's files only; Index.of_cache() adds every checkout of the source cache
(`docs/facts/sources.toml`, update_sources.sh) under its entry's `project`.

    python3 scripts/citations.py [--cache] [--summary] [doc ...]   # records as TSV, or counts per project
    python3 scripts/citations.py --self-test

doc is a key of DOCUMENTS (`spec`, `gap-map-kde`, `platform-facts`, …) or a path; none means every document.

API (run from the repository root):

    DOCUMENTS                  key -> repository path of every document scanned
    SOURCES_SCOPE              the documents whose cited projects must be pinned (check_sources.py)
    REPIN_SCOPE                the documents whose citations are re-pinned (repin_citations.py)
    ABBR                       (token, kind, project, ref, root, defined at): the documents' abbreviations
    Index.of_repo(), Index.of_cache(sources=SOURCES_TOML, cache=None)
    cache_dir(path=None)       the source cache: path, else `NATIVE_THEME_SOURCES`, else DEFAULT_CACHE
    definitions(texts)         the abbreviations' refs as the documents' definitions state them
    scan(text, doc, index, defs=None) -> list[Citation]   one document's text; doc is its short name
                                                          ("gap-map-kde.md"); defs from definitions()
    scan_documents(keys=None, index=None) -> list[Citation]
    projects(citation) -> list[str]               the project names a record names ("A | B" split)

    Citation: doc, line, syntax, project, ref, path, lines, status, kind, refkind, method, note, raw,
              file (the repository path where the rules give one: root + path, the URL's path), quote (bool)
    status: resolved | heuristic | ambiguous | unresolved
"""
import collections
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SOURCES_TOML = Path("docs/facts/sources.toml")
DEFAULT_CACHE = "/run/media/tibi/work/cargo-build/native-theme-sources"


def cache_dir(path=None):
    """The source cache: path, else `NATIVE_THEME_SOURCES`, else DEFAULT_CACHE."""
    return Path(path or os.environ.get("NATIVE_THEME_SOURCES") or DEFAULT_CACHE)

PREFIX = "todo_v0.7.1_model-completeness-"
PLATFORMS = ("gnome", "ios", "kde", "macos", "material", "windows")
DOCUMENTS = {"spec": f"docs/{PREFIX}spec.md", "spec-widgets": f"docs/{PREFIX}spec-widgets.md"}
DOCUMENTS.update({f"spec-trace-{p}": f"docs/{PREFIX}spec-trace-{p}.md" for p in PLATFORMS})
DOCUMENTS.update({"gap-map": f"docs/{PREFIX}gap-map.md", "gap-map-seeds": f"docs/{PREFIX}gap-map-seeds.md"})
DOCUMENTS.update({f"gap-map-{p}": f"docs/{PREFIX}gap-map-{p}.md" for p in PLATFORMS})
DOCUMENTS.update({"platform-facts": "docs/platform-facts.md", "todo": "docs/todo.md"})
SOURCES_SCOPE = [k for k in DOCUMENTS if k != "todo"]
REPIN_SCOPE = [k for k in DOCUMENTS if k not in ("platform-facts", "todo")]

OWN = "native-theme (own)"
PF = "native-theme (own): docs/platform-facts.md"

# The documents' abbreviations: (token, kind, project, ref, root, defined at). kind: "line" = the token directly
# followed by :L (st:123); "path" = the token is a path prefix (A/w/F:L); "word" = the token, a space, then a path
# (kwin src/x.cpp:L). A ref of "SECTION" depends on the heading (heading_qt_ref); "URL" means the link carries it.
# The ref is the one the definition states at "defined at", read from the documents when they are scanned
# (definitions()); the ref here is what it states as of this table's last edit, used where no document is read.
KDE = "gap-map-kde.md:9-31"
ABBR = [
    # the KDE file's table
    ("st", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezestyle.cpp", KDE),
    ("hp", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezehelper.cpp", KDE),
    ("hp.h", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezehelper.h", KDE),
    ("mt", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezemetrics.h", KDE),
    ("kcfg", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breeze.kcfg", KDE),
    ("shd", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezeshadowhelper.cpp", KDE),
    ("anim", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/animations/breezeanimations.cpp", KDE),
    ("tam", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezetoolsareamanager.cpp", KDE),
    ("deco", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kdecoration/breezedecoration.cpp", KDE),
    ("deco.h", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kdecoration/breeze.h", KDE),
    ("dbtn", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kdecoration/breezebutton.cpp", KDE),
    ("dkcfg", "line", "KDE/breeze", "c97883a8 (v6.8.2)", "kdecoration/breezesettingsdata.kcfg", KDE),
    ("kcs", "line", "KDE/kcolorscheme", "b01fde30 (v6.31.0)", "src/kcolorscheme.cpp", KDE),
    ("khint", "line", "KDE/plasma-integration", "b8233c2a (v6.8.2)", "qt6/src/platformtheme/khintssettings.cpp",
     KDE),
    ("kfont", "line", "KDE/plasma-integration", "b8233c2a (v6.8.2)", "qt6/src/platformtheme/kfontsettingsdata.cpp",
     KDE),
    ("kmw", "line", "KDE/kwidgetsaddons", "259b994c (v6.31.0)", "src/kmessagewidget.cpp", KDE),
    ("kie", "line", "KDE/kiconthemes", "420364ec (v6.31.0)", "src/kiconeffect.cpp", KDE),
    ("kit", "line", "KDE/kiconthemes", "420364ec (v6.31.0)", "src/kicontheme.cpp", KDE),
    ("ktb", "line", "KDE/kxmlgui", "e47ccaf5 (v6.31.0)", "src/ktoolbar.cpp", KDE),
    ("kwa", "word", "KDE/kwidgetsaddons", "259b994c (v6.31.0)", "src/", KDE),
    ("kwin", "word", "KDE/kwin", "f130ab33 (v6.8.2)", "", KDE),
    ("KWin", "word", "KDE/kwin", "f130ab33", "", "gap-map-kde.md:1291"),
    ("kirigami", "word", "KDE/kirigami", "eecf183a (v6.31.0)", "", KDE),
    ("qqc2", "word", "KDE/qqc2-desktop-style", "2efe38d5 (v6.31.0)", "", KDE),
    ("kqc", "word", "KDE/kquickcharts", "61fc0d8d (v6.31.0)", "", "gap-map-kde.md:1078"),
    ("ka/", "path", "KDE/kirigami-addons", "4668f4ca (v1.15.0)", "src/", "gap-map-kde.md:1316-1320"),
    ("qtbase", "word", "qt/qtbase", "v6.12.0", "", "gap-map-kde.md:756"),
    ("qtdeclarative", "word", "qt/qtdeclarative", "v6.12.0", "", "gap-map-kde.md:1029"),
    # the GNOME file's table and its later tables
    ("A/w/", "path", "GNOME/libadwaita", "c9ab1186 (1.10.0)", "src/stylesheet/widgets/", "gap-map-gnome.md:11"),
    ("A/s/", "path", "GNOME/libadwaita", "c9ab1186 (1.10.0)", "src/stylesheet/", "gap-map-gnome.md:12"),
    ("A/c/", "path", "GNOME/libadwaita", "c9ab1186 (1.10.0)", "src/", "gap-map-gnome.md:13"),
    ("A/", "path", "GNOME/libadwaita", "c9ab1186 (1.10.0)", "", "gap-map-gnome.md:1003 (A/NEWS)"),
    ("GDK/", "path", "GNOME/gtk", "2ea935e3 (4.24.1)", "gdk/", "gap-map-gnome.md:15"),
    ("G3/", "path", "GNOME/gtk", "3.24.52", "gtk/", "gap-map-gnome.md:1002"),
    ("G/", "path", "GNOME/gtk", "2ea935e3 (4.24.1)", "gtk/", "gap-map-gnome.md:14"),
    ("S/", "path", "GNOME/gsettings-desktop-schemas", "51.0", "schemas/", "gap-map-gnome.md:16"),
    ("M/", "path", "GNOME/mutter", "9134e3a5 (51.1)", "", "gap-map-gnome.md:17; gap-map-seeds.md:587"),
    ("GS-data/", "path", "GNOME/gnome-shell", "33da0685 (51.1)", "data/",
     "gap-map-gnome.md:17; gap-map-seeds.md:588"),
    ("GS/", "path", "GNOME/gnome-shell", "33da0685 (51.1)", "js/ui/", "gap-map-gnome.md:17, :685"),
    ("LS/", "path", "GNOME/libspelling", "0.4.10", "", "gap-map-gnome.md:1004"),
    ("GSD/", "path", "GNOME/gnome-settings-daemon", "4a75955f (51.1)", "", "gap-map-gnome.md:1238"),
    ("XDPG/", "path", "GNOME/xdg-desktop-portal-gnome", "99446c9c (51.0)", "", "gap-map-gnome.md:1239"),
    ("gsv/", "path", "GNOME/gtksourceview", "c7b52cf9 (5.22.0)", "", "gap-map-seeds.md:545"),
    ("LP/", "path", "GNOME/libpanel", "ed3e78f3 (1.10.4)", "src/", "gap-map-seeds.md:589"),
    ("P/", "path", "GNOME/pango", "d360f140 (1.58.2)", "pango/", "gap-map-seeds.md:610"),
    ("fc/", "path", "fontconfig", "2.18.3", "conf.d/", "gap-map-seeds.md:608"),
    # the seeds file's tables (KDE side)
    ("ksh/", "path", "KDE/syntax-highlighting", "6d8ab794 (v6.31.0)", "", "gap-map-seeds.md:546"),
    ("kte/", "path", "KDE/ktexteditor", "ea14509c (v6.31.0)", "", "gap-map-seeds.md:547"),
    ("qt/", "path", "qt/qtbase", "SECTION", "",
     "gap-map-seeds.md:548 (src/gui/text/) / :578 (src/widgets/) / :609 (src/gui/text/)"),
    ("kate/", "path", "KDE/kate", "2b6465a8 (v26.08.2)", "apps/lib/", "gap-map-seeds.md:582"),
    ("konsole/", "path", "KDE/konsole", "c21993d0 (v26.08.2)", "src/", "gap-map-seeds.md:583"),
    ("kdenlive/", "path", "KDE/kdenlive", "ef543982 (v26.08.2)", "", "gap-map-seeds.md:584"),
    ("kddw/", "path", "KDAB/KDDockWidgets", "674f2845 (v2.5.1)", "src/", "gap-map-seeds.md:585"),
    ("kwin/", "path", "KDE/kwin", "f130ab33 (v6.8.2)", "src/", "gap-map-seeds.md:580"),
    # Windows
    ("MUX/", "path", "microsoft/microsoft-ui-xaml", "ba3a8d59 (winui3/release/2.5.1)", "controls/dev/",
     "gap-map-windows.md:9"),
    ("MUX", "word", "microsoft/microsoft-ui-xaml", "ba3a8d59 (winui3/release/2.5.1)", "controls/dev/",
     "gap-map-windows.md:9"),
    ("CTK", "word", "CommunityToolkit/Windows", "a6b4dc45 (v8.2.251219)", "components/",
     "gap-map-windows.md:433-434"),
    ("labs/", "path", "CommunityToolkit/Labs-Windows", "d620e8b5 (release/weekly/251217)",
     "components/MarkdownTextBlock/src/", "gap-map-seeds.md:551"),
    # Material
    ("v0_192/", "path", "material-components/material-web", "b4de401e (v2.5.0)", "tokens/versions/v0_192/",
     "gap-map-material.md:12"),
    ("latest/", "path", "material-components/material-web", "b4de401e (v2.5.0)", "tokens/versions/latest/sass/",
     "gap-map-material.md:13"),
    ("mw/", "path", "material-components/material-web", "b4de401e (v2.5.0)", "", "gap-map-material.md:14"),
    ("compose-android/", "path", "androidx/androidx", "e4d933c9",
     "compose/material3/material3/src/androidMain/kotlin/androidx/compose/material3/", "gap-map-material.md:16"),
    ("compose/", "path", "androidx/androidx", "e4d933c9",
     "compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/", "gap-map-material.md:15"),
    ("ax/", "path", "androidx/androidx", "e4d933c9", "", "gap-map-material.md:789"),
    ("fb/", "path", "AOSP platform/frameworks/base", "94b4c163 (android-17.0.0_r1)", "", "gap-map-material.md:790"),
    ("wms/", "path", "AOSP platform/frameworks/base", "94b4c163 (android-17.0.0_r1)", "libs/WindowManager/Shell/",
     "gap-map-material.md:791"),
    ("aosp/", "path", "AOSP platform/frameworks/base", "94b4c163 (android-17.0.0_r1)", "data/fonts/",
     "gap-map-seeds.md:613"),
    ("mdc/", "path", "material-components/material-components-android", "URL", "",
     "platform-facts.md link texts only"),
]
WORD_ABBR = {a[0]: a for a in ABBR if a[1] == "word"}
LINE_ABBR = {a[0]: a for a in ABBR if a[1] == "line"}
PATH_ABBR = sorted([a for a in ABBR if a[1] == "path"], key=lambda a: -len(a[0]))

# A project named without a commit at the citation, whose commit the KDE file defines elsewhere.
DOCDEFS = {"kiconthemes": ("420364ec", "gap-map-kde.md:27"), "xdg-desktop-portal-kde": ("5b33864b", "gap-map-kde.md:756")}

# The seeds file's sections that define `qt/` (heading_qt_ref): the words of the heading, the definition's line.
QT_SECTIONS = ((("code editor", "rich text"), "Code editor and rich text", 548),
               (("drop zone", "dialog placement"), "Drop zones and dialog placement", 578),
               (("font", "mirroring"), "Fonts and mirroring", 609))
QT_TABLE_REF = {548: "v6.12.0", 578: "v6.12.0", 609: "v6.12.0"}

# name@sha and Org/name @ sha: the short names (gap-map-kde.md:1029, :1281-1286; WebKit: gap-map-macos.md:989)
ALIAS = {
    "kirigami": "KDE/kirigami", "qqc2": "KDE/qqc2-desktop-style", "qqc2-desktop-style": "KDE/qqc2-desktop-style",
    "kxmlgui": "KDE/kxmlgui", "kconfigwidgets": "KDE/kconfigwidgets", "kitemviews": "KDE/kitemviews",
    "plasma-desktop": "KDE/plasma-desktop", "kwin": "KDE/kwin", "breeze": "KDE/breeze",
    "kcolorscheme": "KDE/kcolorscheme", "kiconthemes": "KDE/kiconthemes", "kio": "KDE/kio",
    "kdecoration": "KDE/kdecoration", "plasma-integration": "KDE/plasma-integration",
    "kwidgetsaddons": "KDE/kwidgetsaddons", "libplasma": "KDE/libplasma",
    "xdg-desktop-portal-kde": "KDE/xdg-desktop-portal-kde", "kcompletion": "KDE/kcompletion",
    "ktexteditor": "KDE/ktexteditor", "sonnet": "KDE/sonnet", "kquickcharts": "KDE/kquickcharts",
    "kquickimageeditor": "KDE/kquickimageeditor", "plasma-workspace": "KDE/plasma-workspace", "WebKit": "WebKit/WebKit",
    "androidx": "androidx/androidx",
}

# The 0.8.0 session's scratchpad checkouts that the catalogue cites as `src/<name>/…` (catalogue:8193-8347):
# folder -> (project, that checkout's commit, as the inventory read it on 2026-10-29)
SCRATCHPAD = {
    "breeze": ("KDE/breeze", "75561b7f32f3121603a0f2c8ea929b20a0d6c311"),
    "kcolorscheme": ("KDE/kcolorscheme", "8bb948c4e768800e456b7e8afe0938233a3348a9"),
    "kconfigwidgets": ("KDE/kconfigwidgets", "939320a3371d92ebb7a87a8a955cc0e7fae5580f"),
    "kiconthemes": ("KDE/kiconthemes", "60bdbcdd55a5b8c35e6f546e0cdfa5ea483e6fb8"),
    "kirigami": ("KDE/kirigami", "81ada72b021fe7e34ec4589c9a0a66dc5cf4dbe8"),
    "kwidgetsaddons": ("KDE/kwidgetsaddons", "17cca25bc8ca08063329c9432b2f96a03230bff5"),
    "kwin": ("KDE/kwin", "625e3132326c9ff181a9af342be59fd92f05d8fa"),
    "kxmlgui": ("KDE/kxmlgui", "b5aa1026102a2eb332a07ca629e785535d49f8a7"),
    "mux": ("microsoft/microsoft-ui-xaml", "8463f45162149de0ec3ad7df752596893fe3e13e"),
    "plasma-integration": ("KDE/plasma-integration", "46eddde9d38cf907786e567dd4a942140bb1f797"),
    "plasma-workspace": ("KDE/plasma-workspace", "30beb0095873364c6b34ebcfd292ecafd4db7f61"),
    "qqc2-desktop-style": ("KDE/qqc2-desktop-style", "305eb2c44fdc3eb38eebea3b0e6b493cf12d1e5e"),
}

DOC_SITE = [
    # (regex on host+path, canonical name, kind)
    (r"learn\.microsoft\.com", "learn.microsoft.com", "documentation site"),
    (r"support\.microsoft\.com", "support.microsoft.com", "documentation site"),
    (r"fluent2\.microsoft\.design", "fluent2.microsoft.design", "documentation site"),
    (r"microsoft\.github\.io/windows-docs-rs", "microsoft.github.io/windows-docs-rs", "documentation site"),
    (r"developer\.apple\.com/(tutorials/data/)?design/human-interface-guidelines", "developer.apple.com HIG",
     "documentation site"),
    (r"developer\.apple\.com/videos", "developer.apple.com WWDC videos", "documentation site"),
    (r"developer\.apple\.com/fonts", "developer.apple.com fonts", "documentation site"),
    (r"developer\.apple\.com", "developer.apple.com documentation", "documentation site"),
    (r"support\.apple\.com", "support.apple.com", "documentation site"),
    (r"discussions\.apple\.com", "discussions.apple.com", "forum"),
    (r"m3\.material\.io|^(?:https?://)?m3\.io\b", "m3.material.io", "documentation site"),
    (r"m2\.material\.io", "m2.material.io", "documentation site"),
    (r"developer\.android\.com", "developer.android.com", "documentation site"),
    (r"developers\.google\.com", "developers.google.com", "documentation site"),
    (r"develop\.kde\.org", "develop.kde.org", "documentation site"),
    (r"api\.kde\.org", "api.kde.org", "documentation site"),
    (r"doc\.qt\.io", "doc.qt.io", "documentation site"),
    (r"developer\.gnome\.org", "developer.gnome.org", "documentation site"),
    (r"docs\.gtk\.org", "docs.gtk.org", "documentation site"),
    (r"gnome\.pages\.gitlab\.gnome\.org", "gnome.pages.gitlab.gnome.org", "documentation site"),
    (r"release\.gnome\.org", "release.gnome.org", "documentation site"),
    (r"flatpak\.github\.io", "flatpak.github.io", "documentation site"),
]
CODE_HOST = re.compile(r"(?:https?://)?(github\.com|gitlab\.gnome\.org|invent\.kde\.org|raw\.githubusercontent\.com|"
                       r"chromium\.googlesource\.com|android\.googlesource\.com|searchfox\.org|codebrowser\.dev)/"
                       r"([^/\s]+/[^/\s#]+?)(?:\.git)?(?:/-)?(?:/(blob|tree|raw|commit|blame|\+|commits|issues|pull|"
                       r"discussions)/?([^/\s#)]*))?(/[^\s#)]*)?(#L?\d+(?:-L?\d+)?)?(?=[\s)`>\]|,;]|$)")

RE_MDLINK = re.compile(r"\[([^\]]{1,300})\]\((https?://[^)\s]+)\)")
RE_RELLINK = re.compile(r"\[([^\]]{1,300})\]\((?!https?:)([\w./@-]+\.\w+)(#L\d+(?:-L\d+)?)?\)")
RE_BRACE = re.compile(r"(?<![\w/.-])[\w./-]*\{[\w.,-]+\}[\w./-]*\.\w+(?::\d+(?:[-–]\d+)?)?")
RE_URL = re.compile(r"https?://[^\s)>\]`\"'|]+")
RE_BAREDOMAIN = re.compile(r"(?<![\w/.@-])((?:[a-z0-9-]+\.)+(?:com|org|io|design|dev)(?![\w.-]*\w)(?:/[^\s`),;|]*)?)")
EXTSET = set(("c h cpp hpp cc cxx mm m swift scss sass css qml xaml cs idl kt java ts js mjs json xml in ui kcfg colors "
              "toml rs md txt conf theme frag vert svg interface desktop build yml yaml sh plist py gschema override "
              "kvconfig html gradle properties cmake").split())
RE_TOKEN = re.compile(
    r"(?<![\w/.@#-])"
    r"(?P<path>(?:\.?[\w.+@~-]+/)*[\w.+@~-]+)"
    r"(?P<lines>:\d+(?:[-–]\d+)?(?:,\s*:?\d+(?:[-–]\d+)?)*)?"
    r"(?![\w/])")
RE_CONT = re.compile(r"(?<=[`\s(,;])(?P<lines>:\d+(?:[-–]\d+)?)(?=[`\s),;.]|$)")
RE_PROJSHA = re.compile(r"(?P<name>(?:[A-Za-z][\w.-]*/)?[A-Za-z][\w.+-]*) ?@ ?`?(?P<sha>[0-9a-f]{7,40})`?")
RE_PFLINE = re.compile(r"(?:docs/)?platform-facts(?:\.md)?(?:@(?P<sha>[0-9a-f]{7,40}))?"
                       r"(?::(?P<line>\d+(?:[-–]\d+)?)| ?(?P<sec>§\s?[\d.]+))")
RE_APPLEPATH = re.compile(r"(?<![\w/.-])(?P<p>(?:uikit|appkit|swiftui|foundation|coretext|avkit|localauthentication|"
                          r"bundleresources|quartzcore|coregraphics|corefoundation|applicationservices|accessibility|"
                          r"xcode|widgetkit|mapkit|photokit|visionkit|webkit|uniformtypeidentifiers|corelocation|"
                          r"usernotifications)/[\w./()\-:_?,]+)")
RE_TITLED = re.compile(r"(?P<site>Learn|Support|AS(?:-[\w-]+)?|IPUG-\w+|HIG(?: catalogue)?|legacy HIG|current HIG|Apple|"
                       r"WWDC\d\d-\d+|m3\.io|a-ref|ndk|c-emoji|CAM|GFI|ASF|CT|GN47|GTKgal|ADWgal)\b"
                       r"(?P<rest>\s+\*[^*]{1,120}\*|\s+[A-Z][\w\-]*(?: [A-Z][\w\-]*){0,3}|\s+`[^`]{1,120}`)?")
RE_WORD = re.compile(r"(?:^|[\s`(;,])(?:KDE/)?(?P<w>kwin|KWin|kwa|kqc|MUX|CTK|qtbase|qtdeclarative|kirigami|qqc2|iced|"
                     r"gpui|egui|gpui-component|kiconthemes|libplasma|xdg-desktop-portal-kde|frameworkintegration)"
                     r"(?P<ver> v?\d+\.\d+(?:\.\d+)?)?(?: tree)?\s*`?$")
RE_TRAILING_REF = re.compile(r"`?\)?(?:\s+at\s+(?:[A-Za-z][\w-]+\s+)?`?(v?\d+\.\d+(?:\.\d+)?|[0-9a-f]{8,40})|"
                             r",\s+`?(v\d+\.\d+(?:\.\d+)?))\b(?!\.\d)")
RE_TOOLKIT_NEARBY = re.compile(r"(gpui-component(?:[ -]\d+\.\d+\.\d+)?|gpui-pre(?:-macos)?(?:[ -]\d+\.\d+\.\d+)?|"
                               r"GPUI Kit(?: \d+\.\d+\.\d+)?|gpui(?:[ -]\d+\.\d+\.\d+)?|iced(?:_\w+)?(?:[ -]\d+\.\d+\.\d+)?|"
                               r"egui(?:[ -]\d+\.\d+\.\d+)?|epaint|winit(?:[ -]\d+\.\d+\.\d+)?|cosmic-text)")
OWN_TOP = re.compile(r"(native-theme|connectors|tools|docs|scripts|\.github|crates|evidence|showcase|native-theme-build)/")
OWN_NAMES = ("platform-facts.md", "property-registry.toml", "inheritance-rules.toml", "CHANGELOG.md", "README.md",
             "Cargo.toml")
TOOLKITS = ("iced", "gpui", "egui", "gpui-component")
QUOTE_SECTION = "Section 2: platform-facts.md"   # the seeds file's excerpts of platform-facts cells


@dataclass
class Citation:
    doc: str
    line: int
    syntax: str
    project: str
    ref: str
    path: str
    lines: str
    status: str
    kind: str = ""
    refkind: str = "none"
    method: str = ""
    note: str = ""
    raw: str = ""
    file: str = ""
    quote: bool = False


FIELDS = ["doc", "line", "syntax", "kind", "project", "ref", "refkind", "path", "file", "lines", "method", "status",
          "note", "quote", "raw"]


def is_sha(s):
    return bool(re.fullmatch(r"[0-9a-f]{7,40}", s or ""))


def refkind(ref):
    if not ref:
        return "none"
    r = ref.split(" ")[0]
    if is_sha(r):
        return "sha"
    if r in ("master", "main", "dev", "refs"):
        return "branch"
    if re.fullmatch(r"v?[\d.]+(-\w+)?", r) or re.fullmatch(r"\d+\.\d+", r):
        return "tag"
    if r == "SECTION":
        return "section-dependent"
    if re.fullmatch(r"\d{4}-\d\d-\d\d", r):
        return "date"
    return "other"


def projects(c):
    """The project names a record names: an ambiguous record's "A | B" is two."""
    return [p.strip() for p in c.project.split("|") if p.strip()]


def join(root, rel):
    return (root.rstrip("/") + "/" + rel).lstrip("/") if root else rel


def classify_url(url):
    """("code", project, ref, path, anchor, host) | ("issue", …) | ("doc", site, "", url, "", kind) | ("other", host, …)"""
    m = CODE_HOST.match(url)
    if m:
        host, repo, verb, ref, rest, anchor = m.groups()
        if host == "raw.githubusercontent.com":
            # raw.githubusercontent.com/<owner>/<repo>/<ref>/<path>, the ref a name, a SHA or refs/heads|tags/<name>
            tail = (f"/{verb}" + (f"/{ref}" if ref else "") if verb else "") + (rest or "")
            parts = tail.lstrip("/").split("/")
            n = 3 if parts[0] == "refs" and len(parts) > 3 else 1
            ref, rest = "/".join(parts[:n]), "/" + "/".join(parts[n:])
        if host == "chromium.googlesource.com" and ref == "refs":
            ref = "refs/heads/main"
        proj = repo
        if host == "invent.kde.org":
            proj = "KDE/" + repo.split("/")[1]
        if host == "gitlab.gnome.org":
            proj = "GNOME/" + repo.split("/")[1]
        if repo == "tiborgats/native-theme":
            proj = OWN  # this repository (a CI run's evidence link, a file at a commit): never a pinned source
        if host == "searchfox.org":
            proj = "mozilla/gecko-dev (searchfox mozilla-central)"
            ref = "live"
        if verb in ("issues", "pull", "discussions"):
            return ("issue", proj, "", "", "", host)
        return ("code", proj, ref or "", (rest or "").lstrip("/"), (anchor or "").replace("#", ""), host)
    for rx, canon, kind in DOC_SITE:
        if re.search(rx, url):
            return ("doc", canon, "", url, "", kind)
    host = re.sub(r"^https?://", "", url).split("/")[0]
    return ("other", host, "", url, "", "web page")


def heading_qt_ref(headings, defs=None):
    h = " ".join(headings).lower()
    defs = defs or {}
    for words, title, line in QT_SECTIONS:
        if any(w in h for w in words):
            return defs.get(("qt/", line), QT_TABLE_REF[line]), f'section "{title}" (seeds :{line})'
    refs = sorted({defs.get(("qt/", line), QT_TABLE_REF[line]) for _, _, line in QT_SECTIONS})
    return " | ".join(refs), "qt/ outside a seeds-defined section"


# -- the documents' definitions ----------------------------------------------------------------------------------------

LOC_RX = re.compile(r"(?:([\w.-]+\.md))?:(\d+)(?:-(\d+))?")
RE_DEF_SHA = re.compile(r"(?<![\w@])([0-9a-f]{7,40})(?!\w)")
RE_DEF_TAG = re.compile(r"\btag\s+[`*]*(v?[\w.+/-]*\d[\w.+/-]*?)[`*]*(?=[\s,;:)]|$)|\*\*(v?\d+(?:\.\d+)+)\*\*")


def locations(where):
    """[(document, first line, last line)] of a `defined at` text: `gap-map-kde.md:9-31`, `x.md:17, :685`."""
    out, doc = [], ""
    for d, a, b in LOC_RX.findall(where or ""):
        doc = d or doc
        if doc:
            out.append((doc, int(a), int(b or a)))
    return out


def _token_at(lines, token):
    """(line index, column) of the token's first occurrence, a backticked one preferred, or None. A path
    abbreviation (`MUX/`) may be defined by its name alone (`"MUX" = …`)."""
    rxs = [re.compile(r"`" + re.escape(token) + (r"" if token.endswith("/") else r"`")),
           re.compile((r"(?<![\w/.@-])" if token.endswith("/") else r"(?<![\w.@-])") + re.escape(token)
                      + (r"" if token.endswith("/") else r"(?![\w-])"))]
    if token.endswith("/"):
        rxs.append(re.compile(r"(?<![\w/.@-])" + re.escape(token[:-1]) + r"(?![\w/-])"))
    for rx in rxs:
        for i, s in enumerate(lines):
            m = rx.search(s)
            if m:
                return i, m.start()
    return None


def _ref_near(lines, i, col):
    """The SHA and the tag a definition states for the token at lines[i][col]: the first after it in its clause
    (to a `;` or a sentence's end, a table row's end), else the last before it (a glued `name@sha` and a `tag
    object` SHA name something else)."""
    after = lines[i][col:] if lines[i].lstrip().startswith("|") else lines[i][col:] + "\n" + "\n".join(lines[i + 1:])
    after = re.split(r";|\.\s", after, maxsplit=1)[0]
    before = "\n".join(lines[:i]) + "\n" + lines[i][:col]

    def shas(text):
        return [m.group(1) for m in RE_DEF_SHA.finditer(text) if not text[:m.start()].rstrip("` ").endswith("object")]

    def tags(text):
        return [m.group(1) or m.group(2) for m in RE_DEF_TAG.finditer(text)]

    a_sha, a_tag = shas(after), tags(after)
    if a_sha or a_tag:
        sha, tag = (a_sha or [""])[0], (a_tag or [""])[0]
    else:
        sha, tag = (shas(before) or [""])[-1], (tags(before) or [""])[-1]
    if sha and tag:
        return f"{sha[:8]} ({tag})"
    return sha[:8] or tag


def read_ref(texts, where, token):
    """(ref, (document, line)) the documents' definition at `where` states for token: the first location naming the
    token with a SHA, else the first naming it with a tag; (None, (document, line)) where it names the token and no
    ref; (None, None) where no location names it. texts: short name -> text."""
    found, at = [], None
    for doc, a, b in locations(where):
        if doc not in texts:
            continue
        lines = texts[doc].split("\n")[a - 1:b]
        hit = _token_at(lines, token)
        if hit is None:
            continue
        at = at or (doc, a + hit[0])
        ref = _ref_near(lines, *hit)
        if ref:
            found.append((ref, (doc, a + hit[0])))
    with_sha = [f for f in found if ref_has_sha(f[0])]
    if with_sha or found:
        return (with_sha or found)[0]
    return None, at


def ref_has_sha(ref):
    return bool(RE_DEF_SHA.search(ref or ""))


def definitions(texts):
    """The refs the documents' definitions state now (texts: short name -> text), so that a re-pinned definition is
    read as it stands: token -> ref for every abbreviation of WORD_ABBR, LINE_ABBR and PATH_ABBR and every DOCDEFS
    project, and ("qt/", line) for each seeds section of QT_SECTIONS. A definition that names its token and no ref
    ("same commit", "its …") takes the ref of the nearest definition of the same project above it in the same
    document; a token no document defines keeps the table's ref (ABBR, DOCDEFS)."""
    out, placed = {}, []
    entries = [(a[0], a[2], a[5]) for a in list(WORD_ABBR.values()) + list(LINE_ABBR.values()) + PATH_ABBR
               if a[3] not in ("SECTION", "URL")]
    entries += [(w, ALIAS[w], d[1]) for w, d in DOCDEFS.items()]
    pending = []
    for token, project, where in entries:
        ref, at = read_ref(texts, where, token)
        if ref:
            out[token] = ref
            placed.append((at, project, ref))
        elif at:
            pending.append((token, project, at))
    for token, project, (doc, line) in pending:
        above = [(ln, ref) for (d, ln), p, ref in placed if d == doc and p == project and ln < line]
        if above:
            out[token] = max(above)[1]
    for _, _, line in QT_SECTIONS:
        ref, _ = read_ref(texts, f"gap-map-seeds.md:{line}", "qt/")
        if ref:
            out[("qt/", line)] = ref
    return out


class Index:
    """The files of each project with a checkout, and of this repository, for the existence checks."""

    def __init__(self, own=(), checkouts=None):
        self.files = {}           # project -> set of paths
        self.heads = {}           # project -> the checkout's commit
        self.basenames = collections.defaultdict(set)
        self._dirs, self._suffixes = {}, {}
        self.own = set(own)
        self.add(OWN, self.own)
        for proj, (files, head) in (checkouts or {}).items():
            self.add(proj, files, head)

    def add(self, proj, files, head=""):
        s = self.files.setdefault(proj, set())
        s.update(f for f in files if f)
        if head:
            self.heads[proj] = head
        self._dirs.pop(proj, None)
        self._suffixes.pop(proj, None)
        for f in s:
            self.basenames[os.path.basename(f)].add(proj)

    def path_in(self, proj, path, root=""):
        """True / False when the project has a checkout, None when it has none."""
        s = self.files.get(proj)
        if s is None:
            return None
        full = join(root, path).lstrip("/")
        if full in s:
            return True
        if proj not in self._dirs:
            dirs, suffixes = set(), set()
            for f in s:
                parts = f.split("/")
                for k in range(1, len(parts)):
                    dirs.add("/".join(parts[:k]))
                    suffixes.add("/".join(parts[k:]))
            self._dirs[proj], self._suffixes[proj] = dirs, suffixes
        # a directory, or a path given relative to some subfolder
        return full in self._dirs[proj] or full in self._suffixes[proj]

    @staticmethod
    def git_files(path):
        try:
            out = subprocess.run(["git", "-C", str(path), "ls-files"], capture_output=True, text=True, check=True)
            head = subprocess.run(["git", "-C", str(path), "rev-parse", "HEAD"], capture_output=True, text=True,
                                  check=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            return None
        return out.stdout.split("\n"), head.stdout.strip()

    @classmethod
    def of_repo(cls, repo=REPO):
        got = cls.git_files(repo)
        return cls(got[0] if got else ())

    @classmethod
    def of_cache(cls, sources=SOURCES_TOML, cache=None, repo=REPO):
        """This repository and every source-cache checkout (`<cache>/<id>`) under its entry's project."""
        import tomllib
        cache = cache_dir(cache)
        index = cls.of_repo(repo)
        path = sources if Path(sources).is_absolute() else Path(repo) / sources
        data = tomllib.loads(Path(path).read_text())
        for e in data.get("source", []):
            got = cls.git_files(cache / e["id"]) if (cache / e["id"] / ".git").exists() else None
            if got and e.get("project"):
                index.add(e["project"], got[0], got[1])
        return index


class Unit:
    """A table cell or a prose paragraph: the scope a project prefix carries over in."""

    def __init__(self):
        self.last = None       # the last resolved code citation (project, ref, root)
        self.lastfile = None   # the last citation's (project, ref, path, file), for a :L continuation


class Scanner:
    def __init__(self, doc, index, defs=None):
        self.doc = doc
        self.index = index
        self.defs = defs or {}
        self.records = []
        self.section = ""
        self.linectx = ""
        self.nextline = ""
        self.table_header = ""
        self.docdefault = "windows" if doc.endswith("gap-map-windows.md") else "pf" if doc == "platform-facts.md" else ""

    def add(self, ln, syntax, raw, project="", ref="", path="", lines="", method="", status="", note="", kind="",
            file=None):
        quote = self.doc.endswith("gap-map-seeds.md") and ("G36" in self.section or QUOTE_SECTION in self.section)
        if quote:
            note = "QUOTE of a platform-facts cell (excerpt, may be cut mid-URL); " + note
        self.records.append(Citation(self.doc, ln, syntax, project, ref, path, lines, status, kind, refkind(ref), method,
                                     note, raw.strip()[:200].replace("\t", " "), path if file is None else file, quote))

    def ref(self, a):
        """An abbreviation's ref: its definition's in the documents (definitions()), else the table's."""
        return self.defs.get(a[0], a[3])

    # -- path tokens --------------------------------------------------------------------------------------------------

    def resolve_token(self, ln, unit, text, m, headings):
        idx = self.index
        path = m.group("path")
        lines = (m.group("lines") or "").lstrip(":")
        start = m.start()
        before = text[max(0, start - 80):start]
        base = path.rstrip(".,;")
        ext = base.rsplit(".", 1)[1].lower() if "." in base.split("/")[-1] else ""
        glued = re.match(r"(.*?)@([0-9a-f]{7,40})$", base)
        if glued:
            base = glued.group(1)
            ext = base.rsplit(".", 1)[1].lower() if "." in base.split("/")[-1] else ""
        if not ("/" in base or ext in EXTSET or base in LINE_ABBR):
            return None
        if not lines and base in LINE_ABBR:
            return None   # the abbreviation named in prose or in its definition table
        if not lines and ext not in EXTSET:
            return None   # slash-separated words (on/off, 10/32) and folders: not file citations
        if not lines and "`" not in text[max(0, start - 1):start + 1] and "/" not in base:
            return None   # a bare word with an extension outside code spans: too noisy
        if (re.fullmatch(r"[\d.]+", base) or base.startswith("www.")
                or re.match(r"(?:[a-z0-9-]+\.)+(com|org|io|design|dev)(/|$)", base)):
            return None
        syntax = None
        proj = ref = root = method = note = ""
        file = None
        status = "resolved"
        # 1. a ref glued to the file
        if glued:
            ref = glued.group(2)
            if base.endswith("platform-facts.md") or base.startswith("docs/"):
                proj, method, syntax = OWN, "file@sha", "own-file@sha:L"
            else:
                proj, method, syntax = "", "file@sha", "file@sha"
                if "Mac" in base:
                    proj = "WebKit/WebKit"
                note = "project not stated; name@sha only"
        # 2. a name@sha (or Org/name @ sha) prefix right before the path
        if not proj:
            mm = None
            for mm2 in RE_PROJSHA.finditer(before):
                mm = mm2
            if mm and before[mm.end():].strip(" `") == "":
                name = mm.group("name")
                short = name.split("/")[-1]
                proj = ALIAS.get(name, ALIAS.get(short, name))
                ref, method, syntax = mm.group("sha"), "inline project@sha prefix", "project@sha path:L"
        # 3. a word right before the path
        if not proj:
            wm = RE_WORD.search(before)
            if wm:
                w = wm.group("w")
                if w in TOOLKITS:
                    own = [f for f in idx.own if f.endswith("/" + base)]
                    if own:
                        proj, method, syntax = OWN, "toolkit word + own connector/showcase file (suffix match)", \
                            "toolkit-word path:L"
                        note = f'"{w}" names the connector/showcase in this repo, not the toolkit; {len(own)} own file(s) match'
                        status = "resolved" if len(own) == 1 else "ambiguous"
                        file = own[0] if len(own) == 1 else base
                    else:
                        proj, method, syntax = "toolkit: " + w, "toolkit word", "toolkit-word path:L"
                        status = "unresolved"
                        note = "toolkit or own connector file: no own file matches; version not stated"
                elif w == "frameworkintegration":
                    proj, syntax = "KDE/frameworkintegration", "project tag `path:L`"
                    ref = (wm.group("ver") or "").strip()
                    method = "project word + tag before the code span"
                    if not ref:
                        status, note = "unresolved", "project named without tag"
                elif w in ("kiconthemes", "libplasma", "xdg-desktop-portal-kde"):
                    proj, syntax = ALIAS[w], "project-word path:L"
                    d = DOCDEFS.get(w)
                    if d:
                        ref = self.defs.get(w, d[0])
                        method = f"project word; commit from the KDE file's definitions ({d[1]})"
                        note = "the commit is defined for the project elsewhere in the KDE file, not at the citation"
                    else:
                        method, status = "project word (no commit)", "unresolved"
                        note = "project named without commit; no definition of a commit for it in the gap maps"
                else:
                    a = WORD_ABBR[w]
                    proj, ref, root = a[2], self.ref(a), a[4]
                    if w == "MUX" and base.startswith("dxaml/"):
                        root = ""
                    if wm.group("ver"):
                        ref = wm.group("ver").strip()
                    method, syntax = f"word abbreviation ({w}, {a[5]})", "abbrev-word path:L"
                    file = join(root, base)
        # 4a. a path-prefix abbreviation before an ellipsis (ax/… material3/Menu.kt)
        if not proj:
            em = re.search(r"(?:^|[\s`(;,])([^\s`(;,]+/)… $", before)
            if em:
                for a in PATH_ABBR:
                    if em.group(1) == a[0]:
                        proj, ref, root = a[2], self.ref(a), ""
                        method, syntax = f"path-prefix abbreviation before an ellipsis ({a[0]}, {a[5]})", \
                            "elided path …/x:L"
                        note = "path elided: the middle of the path is not given; match by suffix"
                        break
        # 4b. a path-prefix abbreviation (A/w/, G/, compose/, ka/ …)
        if not proj:
            for a in PATH_ABBR:
                if base.startswith(a[0]):
                    proj, ref, root = a[2], self.ref(a), a[4]
                    method, syntax = f"path-prefix abbreviation ({a[0]}, {a[5]})", "abbrev-path/F:L"
                    if ref == "SECTION":
                        ref, note = heading_qt_ref(headings, self.defs)
                        method += "; ref by section heading"
                        if "|" in ref:
                            status = "ambiguous"
                    if ref == "URL":
                        status, note = "unresolved", "link text only; the URL carries the commit"
                    file = join(root, base[len(a[0]):])
                    break
        # 5. a line abbreviation (st:L)
        if not proj and base in LINE_ABBR and lines:
            a = LINE_ABBR[base]
            proj, ref, root = a[2], self.ref(a), ""
            method, syntax = f"line abbreviation ({base} = {a[4]}, {a[5]})", "abbrev:L"
            file = a[4]
        # 6. this repository's own files
        if not proj and (OWN_TOP.match(base) or base in OWN_NAMES):
            proj, method, syntax = OWN, "own-repo path", "own-file path:L"
            b2 = base
            if b2 in ("platform-facts.md", "property-registry.toml", "inheritance-rules.toml") or b2.startswith("evidence/"):
                b2 = "docs/" + b2
            file = b2
            at = re.match(r"`?\s*(?:at|@)\s*`?([0-9a-f]{7,40})", text[m.end():m.end() + 30])
            if at:
                ref = at.group(1)
                syntax = "own-file path:L at <sha>"
            if b2 not in idx.own and not any(f.startswith(b2.rstrip("/") + "/") for f in idx.own):
                note = "path not in the repo today"
        if base.startswith("/usr/"):
            proj, method, syntax, status = "installed system file", "absolute path", "system path", "unresolved"
            note = "an installed header; no project or version in the citation"
        # 7. a bare path carried from the unit's last project
        if not proj and unit.last and ("/" in base or ext in EXTSET):
            lp, lr, lroot = unit.last
            ex = idx.path_in(lp, base, lroot)
            if ex is True:
                proj, ref, root, method, syntax = lp, lr, lroot, \
                    "carried from the cell/paragraph (verified in checkout)", "bare path (carried)"
                file = join(lroot, base)
            elif ex is None:
                proj, ref, root, method, syntax = lp, lr, lroot, \
                    "carried from the cell/paragraph (no checkout to verify)", "bare path (carried)"
                status = "heuristic"
                file = join(lroot, base)
            else:
                note = f"carry candidate {lp} rejected: path not in its checkout"
        # 8. the document's base (the Windows file: MUX / CTK)
        if not proj and self.docdefault == "windows" and ("/" in base or ext in ("xaml", "cs", "idl", "h", "cpp")):
            a = idx.path_in("microsoft/microsoft-ui-xaml", base, "controls/dev") or \
                idx.path_in("microsoft/microsoft-ui-xaml", base)
            c = idx.path_in("CommunityToolkit/Windows", base, "components")
            mux, ctk = WORD_ABBR["MUX"], WORD_ABBR["CTK"]
            if a and not c:
                proj, ref, method, syntax = mux[2], self.ref(mux), \
                    f"document citation base (MUX, {mux[5]}; verified in checkout)", "bare path (document base)"
                file = base if base.startswith("dxaml/") else join("controls/dev", base)
            elif c and not a:
                proj, ref, method, syntax = ctk[2], self.ref(ctk), \
                    f"document citation base (CTK, {ctk[5]}; verified in checkout)", "bare path (document base)"
                file = join("components", base)
            elif a and c:
                proj, method, syntax, status = "microsoft/microsoft-ui-xaml | CommunityToolkit/Windows", \
                    "document base", "bare path (document base)", "ambiguous"
                note = "path exists in both MUX and CTK"
        # 9. the name looked up in the checkouts and the repository
        if not proj and (ext in EXTSET or "/" in base):
            cands = {p for p in idx.basenames.get(os.path.basename(base), set()) if idx.path_in(p, base)}
            syntax = "bare file:L" if "/" not in base else "bare path:L"
            if not lines:
                syntax = "bare file (no line)" if "/" not in base else "bare path (no line)"
            scratch = re.match(r"src/([\w-]+)/(.*)", base)
            if len(cands) == 1:
                proj = cands.pop()
                method, status = "name lookup in the local checkouts (unique)", "heuristic"
                head = idx.heads.get(proj, "-")[:8]
                note = f"project by name only; ref not stated (the checkout at {head} is only where it was found)"
                if proj == OWN:
                    same = sorted(f for f in idx.own if f.endswith("/" + base) or f == base)
                    if len(same) > 1:
                        status = "ambiguous"
                        note = f"{len(same)} own files end in {base} (e.g. {', '.join(same[:2])})"
                    else:
                        note = "own file by name: " + (same[0] if same else "?")
                        file = same[0] if same else base
            elif len(cands) > 1:
                proj, method, status = " | ".join(sorted(cands)), "name lookup", "ambiguous"
                note = f"file name exists in {len(cands)} checkouts"
            elif scratch and scratch.group(1) in SCRATCHPAD:
                proj, ref = SCRATCHPAD[scratch.group(1)]
                method, status, syntax = "session-scratchpad path (src/<checkout>/…)", "heuristic", \
                    "scratchpad checkout path:L"
                note = "cites the session scratchpad folder, not the repository; ref = that checkout's commit"
                file = scratch.group(2)
            elif ext == "rs":
                proj, method, status = "toolkit (gpui / gpui-component / iced / egui: named in the prose)", \
                    "not in the repo: a toolkit crate file", "unresolved"
                note = "crate and version only in the surrounding prose, if at all"
            else:
                method, status = "none", "unresolved"
                note = "no prefix, no carry, no checkout has this name"
        trailing = RE_TRAILING_REF.match((text[m.end():] + " " + self.nextline)[:80])
        if trailing and proj != OWN:
            if not ref or method.startswith(("name lookup", "carried")):
                ref = trailing.group(1) or trailing.group(2)
                syntax = "file:L at <ref>"
                if status in ("unresolved", "heuristic") and proj and "|" not in proj:
                    status = "resolved" if not method.startswith("name lookup") else "heuristic"
                    note = 'ref stated after the citation ("at <ref>")' + \
                        ("; project by file-name lookup" if method.startswith("name") else "")
        if proj.startswith("toolkit"):
            found = RE_TOOLKIT_NEARBY.findall(self.linectx)
            if found:
                note = "crates named nearby: " + ", ".join(sorted(set(found))[:5])
        if text[max(0, start - 2):start] == "… " or text[max(0, start - 1):start] == "…":
            syntax = "elided path …/x:L"
            file = ""
            if status == "resolved" and method.startswith("name"):
                status = "heuristic"
            if not proj or status == "unresolved":
                note = (note + "; " if note else "") + "path elided with …"
        if not syntax:
            syntax = "other"
        if not lines and "no line" not in syntax:
            syntax += " (no line)"
        if proj and proj not in (OWN, "installed system file") and status == "resolved" and not ref:
            status, note = "unresolved", (note + "; " if note else "") + "no commit or tag"
        if proj in idx.files and proj != OWN and status in ("resolved", "heuristic"):
            rel = base
            for a in PATH_ABBR:
                if base.startswith(a[0]):
                    rel = base[len(a[0]):]
                    break
            if base in LINE_ABBR:
                rel = LINE_ABBR[base][4]
            if idx.path_in(proj, rel, root) is False and idx.path_in(proj, rel) is False:
                note = (note + "; " if note else "") + f"path NOT found in the {proj} checkout"
        if proj and status in ("resolved", "heuristic") and proj != OWN:
            unit.last = (proj, ref, root)
        if file is None:
            file = base if status != "unresolved" or proj else ""
        unit.lastfile = (proj, ref, base, file or base)
        return dict(syntax=syntax, project=proj, ref=ref, path=base, lines=lines, method=method, status=status,
                    note=note, file=file)

    # -- one cell or paragraph ----------------------------------------------------------------------------------------

    def scan_text(self, ln, text, unit, headings):
        text = text.replace("…/", "… ")  # an elided path: let the token start after the ellipsis
        spans = []  # consumed ranges
        marks = []  # (position, lastfile) set by links and platform-facts references, applied in text order

        def free(a, b):
            return all(b <= s or a >= e for s, e in spans)

        for m in RE_MDLINK.finditer(text):
            url = m.group(2)
            k = classify_url(url)
            if k[0] == "code":
                st, note = ("resolved", "") if k[2] else ("unresolved", "no ref in URL")
                if k[2] and refkind(k[2]) == "branch":
                    st, note = "unresolved", f"URL at a branch ({k[2]}), no commit"
                self.add(ln, "md-link [file:L](code-host URL @ref#L)", m.group(0), k[1], k[2], k[3], k[4], "URL", st,
                         note, kind="code repo")
                marks.append((m.end(), (k[1], k[2], k[3], k[3])))
            elif k[0] == "doc":
                self.add(ln, "md-link [title](doc URL)", m.group(0), k[1], "", url, "", "URL", "unresolved",
                         "documentation page: no commit; read date only where the text states one", kind=k[5], file="")
            elif k[0] == "issue":
                self.add(ln, "md-link [title](issue/PR URL)", m.group(0), k[1], "", url, "", "URL", "resolved",
                         "issue/PR/discussion, not a source file", kind="issue tracker", file="")
            else:
                self.add(ln, "md-link [title](other URL)", m.group(0), k[1], "", url, "", "URL", "unresolved",
                         "web page: no version", kind="web page", file="")
            spans.append((m.start(), m.end()))
        for m in RE_RELLINK.finditer(text):
            if not free(m.start(), m.end()):
                continue
            p, anchor = m.group(2), m.group(3) or ""
            self.add(ln, "md-link [text](relative own path#L)", m.group(0), OWN, "", p,
                     anchor.replace("#L", "").replace("-L", "-"), "relative link", "resolved",
                     "own file, no commit (the citing document's own commit)", kind="own project file")
            marks.append((m.end(), (OWN, "", p, p)))
            spans.append((m.start(), m.end()))
        for m in RE_BRACE.finditer(text):
            if not free(m.start(), m.end()):
                continue
            self.add(ln, "brace-list path {a,b}.ext:L", m.group(0), "", "", m.group(0), "", "brace expansion",
                     "unresolved", "one citation stands for several files; resolve by the cell's prefix and expand",
                     kind="code repo", file="")
            spans.append((m.start(), m.end()))
        for m in RE_URL.finditer(text):
            if not free(m.start(), m.end()):
                continue
            url = m.group(0).rstrip(".,;:")
            k = classify_url(url)
            if k[0] == "code":
                st = "resolved" if k[2] and refkind(k[2]) in ("sha", "tag") else "unresolved"
                self.add(ln, "raw code-host URL", url, k[1], k[2], k[3], k[4], "URL", st,
                         "" if st == "resolved" else "URL at a branch or without ref", kind="code repo")
            elif k[0] == "doc":
                self.add(ln, "raw doc URL", url, k[1], "", url, "", "URL", "unresolved", "documentation page: no commit",
                         kind=k[5], file="")
            elif k[0] == "issue":
                self.add(ln, "raw issue URL", url, k[1], "", url, "", "URL", "resolved", "issue/PR", kind="issue tracker",
                         file="")
            else:
                self.add(ln, "raw other URL", url, k[1], "", url, "", "URL", "unresolved", "web page", kind="web page",
                         file="")
            spans.append((m.start(), m.end()))
        for m in RE_BAREDOMAIN.finditer(text):
            if not free(m.start(), m.end()):
                continue
            d = m.group(1).rstrip(".,;:")
            host = d.split("/")[0]
            if "/" not in d.rstrip("/"):
                continue   # a host named in prose ("read from gitlab.gnome.org at the tag"): not a citation
            if host.endswith((".rs", ".md", ".toml")) or host in ("example.com",):
                continue
            if not re.search(r"\.(com|org|io|design|dev)$", host):
                continue
            if host.count(".") == 1 and host.split(".")[0] in ("md", "values", "role", "light", "dark", "declare",
                                                                "window", "component", "androidx", "token"):
                continue
            k = classify_url("https://" + d)
            if k[0] == "code":
                self.add(ln, "schemeless code-host path", d, k[1], k[2], k[3], k[4], "host path",
                         "resolved" if k[2] else "unresolved",
                         "" if k[2] else "repository named, ref given elsewhere in the sentence (if at all)",
                         kind="code repo")
            elif k[0] in ("doc", "other"):
                self.add(ln, "schemeless doc-site path", d, k[1], "", d, "", "host path", "unresolved",
                         "documentation page: no commit", kind=k[5], file="")
            spans.append((m.start(), m.end()))
        for m in RE_PFLINE.finditer(text):
            if not free(m.start(), m.end()):
                continue
            if m.group("line"):
                syn = "platform-facts.md@<sha>:LINE" if m.group("sha") else "platform-facts.md:LINE"
                self.add(ln, syn, m.group(0), PF, m.group("sha") or "", "docs/platform-facts.md", m.group("line"),
                         "own file", "resolved" if m.group("sha") else "unresolved",
                         "" if m.group("sha") else "line number without the commit it was read at", kind="own project file")
                marks.append((m.end(), (PF, m.group("sha") or "", "docs/platform-facts.md", "docs/platform-facts.md")))
            else:
                self.add(ln, "platform-facts §section", m.group(0), PF, "", "docs/platform-facts.md", m.group("sec"),
                         "own file", "resolved", "section reference (no line)", kind="own project file")
            spans.append((m.start(), m.end()))
        for m in RE_APPLEPATH.finditer(text):
            if not free(m.start(), m.end()):
                continue
            self.add(ln, "Apple doc path (uikit/x/y)", m.group("p"), "developer.apple.com documentation", "", m.group("p"),
                     "", "path shorthand (gap-map-ios.md:10-11; gap-map-macos.md:15-16)", "unresolved",
                     "documentation page: read date only (stated per file/section)", kind="documentation site", file="")
            spans.append((m.start(), m.end()))
        for m in RE_TITLED.finditer(text):
            if not free(m.start(), m.end()):
                continue
            site = m.group("site")
            rest = (m.group("rest") or "").strip()
            if site in ("Apple", "Support", "CT") and not rest:
                continue
            if site == "Apple" and not rest.startswith("`"):
                continue
            if site == "HIG" and not rest:
                continue
            canon = {"Learn": "learn.microsoft.com", "Support": "support.microsoft.com", "m3.io": "m3.material.io",
                     "a-ref": "developer.android.com", "ndk": "developer.android.com", "c-emoji": "developer.android.com",
                     "CAM": "developer.android.com", "GFI": "developers.google.com", "ASF": "developer.apple.com fonts",
                     "CT": "developer.apple.com documentation", "GN47": "release.gnome.org", "GTKgal": "GNOME/gtk",
                     "ADWgal": "GNOME/libadwaita"}.get(site)
            if canon is None:
                if site.startswith(("HIG", "legacy HIG", "current HIG")):
                    canon = "developer.apple.com HIG"
                elif site.startswith(("AS", "IPUG")):
                    canon = "support.apple.com"
                elif site.startswith("WWDC"):
                    canon = "developer.apple.com WWDC videos"
                else:
                    canon = "developer.apple.com documentation"
            gallery = site in ("GTKgal", "ADWgal")
            label = ("Learn *Title*" if site == "Learn" else "HIG <page>" if "HIG" in site
                     else "Apple `x/y`" if site == "Apple" else site + " abbreviation")
            gfile = {"GTKgal": "docs/reference/gtk/visual_index.md", "ADWgal": "doc/widget-gallery.md"}.get(site, "")
            self.add(ln, f"titled page ({label} …)", m.group(0), canon,
                     "4.24.1" if site == "GTKgal" else "1.10.0" if site == "ADWgal" else "", rest.strip("*` "), "",
                     "title or abbreviation", "resolved" if gallery else "unresolved",
                     "gallery file of the tag" if gallery else "page named by title, no URL; read date stated per file/section",
                     kind="code repo" if gallery else "documentation site", file=gfile)
            spans.append((m.start(), m.end()))
        cands = [(m.start(), 0, m) for m in RE_TOKEN.finditer(text)] + [(m.start(), 1, m) for m in RE_CONT.finditer(text)]
        cands.sort(key=lambda c: (c[0], c[1]))
        marks.sort(key=lambda x: x[0])
        for start, typ, m in cands:
            while marks and marks[0][0] <= start:
                unit.lastfile = marks.pop(0)[1]
            if not free(m.start(), m.end()):
                continue
            if typ == 1:
                self.continuation(ln, text, unit, m, spans)
                continue
            r = self.resolve_token(ln, unit, text, m, headings)
            if r is None:
                continue
            kind = "own project file" if r["project"] == OWN else "toolkit" if r["project"].startswith("toolkit") \
                else "code repo"
            self.add(ln, r["syntax"], text[max(0, m.start() - 30):m.end() + 5], r["project"], r["ref"], r["path"],
                     r["lines"], r["method"], r["status"], r["note"], kind=kind, file=r["file"])
            spans.append((m.start(), m.end()))

    def continuation(self, ln, text, unit, m, spans):
        lf = unit.lastfile
        lines = m.group("lines").lstrip(":")
        raw = text[max(0, m.start() - 20):m.end() + 3]
        if not lf and "platform-facts" in self.table_header.lower() and text.strip().startswith(m.group("lines")):
            self.add(ln, "platform-facts.md:LINE (table column \":L\")", text.strip()[:60], PF, "",
                     "docs/platform-facts.md", lines, "table header names platform-facts", "unresolved",
                     "line number without the commit it was read at", kind="own project file")
        elif lf and lf[0]:
            ok = lf[1] or lf[0] == OWN
            self.add(ln, ":L continuation", raw, lf[0], lf[1], lf[2], lines,
                     "continuation of the previous citation in the cell/paragraph", "resolved" if ok else "unresolved",
                     "" if ok else "inherits a citation without commit",
                     kind="own project file" if lf[0] == OWN else "code repo", file=lf[3])
        else:
            self.add(ln, ":L continuation", raw, "", "", "", lines, "continuation", "unresolved",
                     "no previous resolvable citation in the cell/paragraph", file="")
        spans.append((m.start(), m.end()))

    # -- a document ---------------------------------------------------------------------------------------------------

    def scan(self, text):
        headings = []
        unit = Unit()
        prev = ""
        alllines = text.split("\n")
        for ln, s in enumerate(alllines, 1):
            self.linectx = " ".join(alllines[max(0, ln - 4):ln + 2])
            nxt = alllines[ln] if ln < len(alllines) else ""
            self.nextline = nxt.strip() if not nxt.lstrip().startswith("|") else ""
            if s.startswith("#"):
                lvl = len(s) - len(s.lstrip("#"))
                headings = headings[:lvl - 1] + [s.lstrip("#").strip()]
                self.section = " / ".join(headings)
                unit = Unit()
                continue
            if not s.strip():
                unit = Unit()
                self.table_header = ""
                continue
            if re.match(r"\s*\|[\s:|-]+\|\s*$", s):
                self.table_header = prev
                continue
            prev = s
            if s.lstrip().startswith("|"):
                cells = s.strip().strip("|").split("|")
                h0 = self.table_header.strip().strip("|").split("|")[0].strip().lower() if self.table_header else ""
                if h0 in ("platform-facts line", "line(s)") and self.doc.startswith("gap-map"):
                    lm = re.match(r"\s*((?:`?:?\d{2,4}(?:[-–]\d+)?`?(?:,\s*|\s+and\s+)?)+)", cells[0])
                    if lm:
                        for num in re.findall(r"\d{2,4}(?:[-–]\d+)?", lm.group(1)):
                            self.add(ln, "platform-facts.md:LINE (table column)", cells[0].strip()[:60], PF, "",
                                     "docs/platform-facts.md", num, f'table header "{h0}"', "unresolved",
                                     "line number without the commit it was read at (the header names platform-facts; "
                                     "the file says which commit, if anywhere)", kind="own project file")
                        cells[0] = " " * len(lm.group(1)) + cells[0][len(lm.group(1)):]
                for c in cells:
                    self.scan_text(ln, c, Unit(), headings)
                unit = Unit()
            else:
                self.scan_text(ln, s, unit, headings)
        return self.records


def short_name(path):
    return Path(path).name.replace(PREFIX, "")


def scan(text, doc, index, defs=None):
    """The citations of one document's text; doc is its short name ("gap-map-kde.md", "platform-facts.md"); defs
    the refs definitions() read from the documents (none: the tables')."""
    return Scanner(doc, index, defs).scan(text)


def scan_documents(keys=None, index=None, repo=REPO):
    """The citations of the documents (keys of DOCUMENTS or paths), the abbreviations' refs read from every document
    of DOCUMENTS that exists."""
    index = index or Index.of_repo(repo)
    texts = {}
    for p in DOCUMENTS.values():
        if (Path(repo) / p).is_file():
            texts[short_name(p)] = (Path(repo) / p).read_text(encoding="utf-8")
    defs = definitions(texts)
    out = []
    for k in keys or DOCUMENTS:
        path = Path(DOCUMENTS.get(k, k))
        path = path if path.is_absolute() else Path(repo) / path
        out += scan(path.read_text(encoding="utf-8"), short_name(path), index, defs)
    return out


# -- self-test ------------------------------------------------------------------------------------------------------

SELF_TEST = [
    # (doc, line of text, index files {project: [paths]}, expected [(syntax, project, ref, file, lines, status)])
    ("A1 link to a code host", "platform-facts.md",
     "[gtktooltip.c:71-73](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtktooltip.c#L71-L73)", {},
     [("md-link [file:L](code-host URL @ref#L)", "GNOME/gtk", "4.24.1", "gtk/gtktooltip.c", "L71-L73", "resolved")]),
    ("A1 link at a branch", "platform-facts.md",
     "[x.mm](https://github.com/WebKit/WebKit/blob/main/Source/x.mm#L3)", {},
     [("md-link [file:L](code-host URL @ref#L)", "WebKit/WebKit", "main", "Source/x.mm", "L3", "unresolved")]),
    ("A2 raw URL", "todo.md", "see https://github.com/microsoft/WindowsAppSDK/blob/main/specs/themes/ThemeSettings.md", {},
     [("raw code-host URL", "microsoft/WindowsAppSDK", "main", "specs/themes/ThemeSettings.md", "", "unresolved")]),
    ("A2 own CI run URL", "spec-widgets.md", "run [1](https://github.com/tiborgats/native-theme/actions/runs/1)", {},
     [("md-link [file:L](code-host URL @ref#L)", "native-theme (own)", "", "actions/runs/1", "", "unresolved")]),
    ("A3 name@sha path", "gap-map-kde.md", "`kirigami@81ada72b src/templates/NavigationTabBar.qml:180-277`", {},
     [("project@sha path:L", "KDE/kirigami", "81ada72b", "src/templates/NavigationTabBar.qml", "180-277", "resolved")]),
    ("A3 androidx@sha path", "spec.md", "`androidx@e4d933c9 compose/material3/x/Button.kt:12`", {},
     [("project@sha path:L", "androidx/androidx", "e4d933c9", "compose/material3/x/Button.kt", "12", "resolved")]),
    ("A6 file@sha", "gap-map-ios.md", "`RenderThemeMac.mm@de320ba3:3373-3376`", {},
     [("file@sha", "WebKit/WebKit", "de320ba3", "RenderThemeMac.mm", "3373-3376", "resolved")]),
    ("A7 project tag path", "gap-map-kde.md", "frameworkintegration v6.31.0 `src/kstyle/kstyle.cpp:424-427`", {},
     [("project tag `path:L`", "KDE/frameworkintegration", "v6.31.0", "src/kstyle/kstyle.cpp", "424-427", "resolved")]),
    ("B1 line abbreviation and continuation", "gap-map-kde.md", "pub `hp:678-695`, `:700`", {},
     [("abbrev:L", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezehelper.cpp", "678-695", "resolved"),
      (":L continuation", "KDE/breeze", "c97883a8 (v6.8.2)", "kstyle/breezehelper.cpp", "700", "resolved")]),
    ("B2 path abbreviation", "gap-map-gnome.md", "`A/w/_buttons.scss:328-365`", {},
     [("abbrev-path/F:L", "GNOME/libadwaita", "c9ab1186 (1.10.0)", "src/stylesheet/widgets/_buttons.scss", "328-365",
       "resolved")]),
    ("B3 word abbreviation", "gap-map-windows.md", "P: MUX `CommonStyles/ScrollBar_themeresources.xaml:681-689`", {},
     [("abbrev-word path:L", "microsoft/microsoft-ui-xaml", "ba3a8d59 (winui3/release/2.5.1)",
       "controls/dev/CommonStyles/ScrollBar_themeresources.xaml", "681-689", "resolved")]),
    ("B4 project word, commit defined elsewhere", "gap-map-kde.md", "`kiconthemes src/kiconloader.h:193`", {},
     [("project-word path:L", "KDE/kiconthemes", "420364ec", "src/kiconloader.h", "193", "resolved")]),
    ("B5 gallery", "gap-map-gnome.md", "seen in GTKgal", {},
     [("titled page (GTKgal abbreviation …)", "GNOME/gtk", "4.24.1", "docs/reference/gtk/visual_index.md", "",
       "resolved")]),
    ("C2 carry, verified", "gap-map-kde.md",
     "`kirigami@81ada72b src/controls/Heading.qml:35-37`; `src/templates/Heading.qml:66-70`",
     {"KDE/kirigami": ["src/controls/Heading.qml", "src/templates/Heading.qml"]},
     [("project@sha path:L", "KDE/kirigami", "81ada72b", "src/controls/Heading.qml", "35-37", "resolved"),
      ("bare path (carried)", "KDE/kirigami", "81ada72b", "src/templates/Heading.qml", "66-70", "resolved")]),
    ("C3 document base", "gap-map-windows.md", "(`CommonStyles/Button_themeresources.xaml:38`)",
     {"microsoft/microsoft-ui-xaml": ["controls/dev/CommonStyles/Button_themeresources.xaml"],
      "CommunityToolkit/Windows": ["components/Segmented/src/Segmented.xaml"]},
     [("bare path (document base)", "microsoft/microsoft-ui-xaml", "ba3a8d59 (winui3/release/2.5.1)",
       "controls/dev/CommonStyles/Button_themeresources.xaml", "38", "resolved")]),
    ("C4 section-scoped qt/", "gap-map-seeds.md", "`qt/qtextedit.cpp:10`", {},
     [("abbrev-path/F:L", "qt/qtbase", "v6.12.0", "qtextedit.cpp", "10", "resolved")]),
    ("C5 elided path", "gap-map-material.md", "`ax/…/material3/Menu.kt:2390-2391`", {},
     [("elided path …/x:L", "androidx/androidx", "e4d933c9", "", "2390-2391", "resolved")]),
    ("C6 brace list", "gap-map-material.md", "`_md-comp-icon-button-{standard,filled}.scss`", {},
     [("brace-list path {a,b}.ext:L", "", "", "", "", "unresolved")]),
    ("C7 platform-facts column", "gap-map-gnome.md",
     "| platform-facts line | key |\n|---|---|\n| 1061 | `selection_inactive_background` |", {},
     [("platform-facts.md:LINE (table column)", PF, "", "docs/platform-facts.md", "1061", "unresolved")]),
    ("D1 bare file, unique by name", "spec-widgets.md", "(`gtkheaderbar.c:660-671`)",
     {"GNOME/gtk": ["gtk/gtkheaderbar.c"]},
     [("bare file:L", "GNOME/gtk", "", "gtkheaderbar.c", "660-671", "heuristic")]),
    ("D1 bare file, two projects", "spec-widgets.md", "(`_colors.scss:1`)",
     {"GNOME/gtk": ["gtk/theme/_colors.scss"], "GNOME/libadwaita": ["src/stylesheet/_colors.scss"]},
     [("bare file:L", "GNOME/gtk | GNOME/libadwaita", "", "_colors.scss", "1", "ambiguous")]),
    ("D1 at <ref> after the citation", "spec.md", "(`colorsapplicator.cpp:247-284` at v6.8.2)",
     {"KDE/plasma-workspace": ["kcms/colors/colorsapplicator.cpp"]},
     [("file:L at <ref>", "KDE/plasma-workspace", "v6.8.2", "colorsapplicator.cpp", "247-284", "heuristic")]),
    ("D3 scratchpad path", "spec-widgets.md", "(`src/breeze/kstyle/breezehelper.cpp:142-144`)", {},
     [("scratchpad checkout path:L", "KDE/breeze", "75561b7f32f3121603a0f2c8ea929b20a0d6c311", "kstyle/breezehelper.cpp",
       "142-144", "heuristic")]),
    ("E1 Apple path", "gap-map-ios.md", "uikit/uisplitviewcontroller/primarycolumnwidth", {},
     [("Apple doc path (uikit/x/y)", "developer.apple.com documentation", "",
       "", "", "unresolved")]),
    ("E2 HIG title", "gap-map-macos.md", "HIG Alerts", {},
     [("titled page (HIG <page> …)", "developer.apple.com HIG", "", "", "", "unresolved")]),
    ("E3 Learn title", "gap-map-windows.md", "Learn *DWMWINDOWATTRIBUTE*, updated 2026-03-09", {},
     [("titled page (Learn *Title* …)", "learn.microsoft.com", "", "", "", "unresolved")]),
    ("E6 doc link", "platform-facts.md", "[Fluent 2 spec](https://fluent2.microsoft.design/elevation)", {},
     [("md-link [title](doc URL)", "fluent2.microsoft.design", "", "", "", "unresolved")]),
    ("E7 schemeless doc path", "gap-map-gnome.md", "see release.gnome.org/47/ for it", {},
     [("schemeless doc-site path", "release.gnome.org", "", "", "", "unresolved")]),
    ("F1 platform-facts at a commit", "gap-map-seeds.md", "platform-facts.md@fc6109d1:1435", {},
     [("platform-facts.md@<sha>:LINE", PF, "fc6109d1", "docs/platform-facts.md", "1435", "resolved")]),
    ("F3 own path at a commit", "todo.md", "(`native-theme/src/macos.rs:157` at caeddd6b)",
     {OWN: ["native-theme/src/macos.rs"]},
     [("own-file path:L at <sha>", OWN, "caeddd6b", "native-theme/src/macos.rs", "157", "resolved")]),
    ("F5 connector word", "gap-map-gnome.md", "seed: gpui `info/chrome.rs:207`",
     {OWN: ["connectors/native-theme-gpui/examples/showcase/info/chrome.rs"]},
     [("toolkit-word path:L", OWN, "", "connectors/native-theme-gpui/examples/showcase/info/chrome.rs", "207",
       "resolved")]),
    ("G1 toolkit word", "todo.md", "at gpui-component 0.8.1 `dialog/dialog.rs:775-861`", {},
     [("toolkit-word path:L", "toolkit: gpui-component", "", "dialog/dialog.rs", "775-861", "unresolved")]),
    ("G1 toolkit file by extension", "todo.md", "(`tree.rs:90-92`)", {},
     [("bare file:L", "toolkit (gpui / gpui-component / iced / egui: named in the prose)", "", "tree.rs", "90-92",
       "unresolved")]),
    ("seeds quote", "gap-map-seeds.md", "## Section 2: platform-facts.md cells\n\n[x](https://github.com/mozilla/ge", {},
     [("raw code-host URL", "mozilla/ge", "", "", "", "unresolved")]),
]


# The definitions as the documents state them after a re-pin: (document, line) -> text; definitions() reads them.
SELF_TEST_DEFS = {
    ("gap-map-kde.md", 11): "| `st` | `kstyle/breezestyle.cpp` | KDE/breeze @ `aaaaaaaa11` (tag `v9.9.9`) |",
    ("gap-map-kde.md", 12): "| `hp` / `hp.h` | `kstyle/breezehelper.cpp` / `.h` | KDE/breeze @ `bbbbbbbb` |",
    ("gap-map-kde.md", 1317): "**v1.15.0** (tag object `dddddddd`, commit",
    ("gap-map-kde.md", 1318): "`eeeeeeee`, read from the server. **`ka/F:L`** = file `F`, line(s) `L`, at that commit.",
    ("gap-map-kde.md", 1319): "Comparisons cite `kirigami@ffffffff`.",
    ("gap-map-material.md", 15): "| `compose/` | androidx `cccccccc`, `compose/…/material3/` |",
    ("gap-map-material.md", 16): "| `compose-android/` | same commit, `…/androidMain/…` |",
    ("gap-map-seeds.md", 548): "| `qt/F:L` | qtbase tag **v6.11.2**, `src/gui/text/F` |",
}
SELF_TEST_DEFS_WANT = {"st": "aaaaaaaa (v9.9.9)", "hp": "bbbbbbbb", "hp.h": "bbbbbbbb", "ka/": "eeeeeeee (v1.15.0)",
                       "compose/": "cccccccc", "compose-android/": "cccccccc", ("qt/", 548): "v6.11.2"}


def self_test() -> int:
    failures = []
    texts = collections.defaultdict(lambda: [""] * 1400)
    for (doc, ln), line in SELF_TEST_DEFS.items():
        texts[doc][ln - 1] = line
    texts = {d: "\n".join(lines) for d, lines in texts.items()}
    defs = definitions(texts)
    if defs != SELF_TEST_DEFS_WANT:
        failures.append(f"definitions: got {defs}, want {SELF_TEST_DEFS_WANT}")
    for doc, text, want in (("gap-map-kde.md", "`hp:678`; `mt:5`", ["bbbbbbbb", LINE_ABBR["mt"][3]]),
                            ("gap-map-material.md", "`compose-android/Menu.android.kt:3`", ["cccccccc"]),
                            ("gap-map-seeds.md", "`qt/qtextedit.cpp:10`", ["v6.11.2 | v6.12.0"])):
        got = [c.ref for c in scan(text, doc, Index(), defs)]
        if got != want:
            failures.append(f"scan with the documents' definitions, {text!r}: got {got}, want {want}")
    for url, want in (
            ("https://raw.githubusercontent.com/KDE/breeze/c97883a8fe41bce774835c25d7f7e321c71fa618/kstyle/x.cpp",
             ("code", "KDE/breeze", "c97883a8fe41bce774835c25d7f7e321c71fa618", "kstyle/x.cpp", "",
              "raw.githubusercontent.com")),
            ("https://raw.githubusercontent.com/o/r/refs/tags/v1.2/src/a.rs",
             ("code", "o/r", "refs/tags/v1.2", "src/a.rs", "", "raw.githubusercontent.com")),
            ("https://raw.githubusercontent.com/o/r/blob/x.md",
             ("code", "o/r", "blob", "x.md", "", "raw.githubusercontent.com"))):
        got = classify_url(url)
        if got != want:
            failures.append(f"classify_url({url!r}): got {got}, want {want}")
    for label, doc, text, files, want in SELF_TEST:
        own = files.get(OWN, [])
        index = Index(own, {p: (fs, "") for p, fs in files.items() if p != OWN})
        got = [(c.syntax, c.project, c.ref, c.file, c.lines, c.status) for c in scan(text, doc, index)]
        if got != want:
            failures.append(f"{label}: got {got}, want {want}")
        if label == "seeds quote" and not all(c.quote for c in scan(text, doc, index)):
            failures.append(f"{label}: the record is not marked as a quote")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST)} cases)")
    return 1 if failures else 0


def main(argv):
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    cache = "--cache" in args
    summary = "--summary" in args
    docs = [a for a in args if not a.startswith("--")]
    bad = [a for a in args if a.startswith("--") and a not in ("--cache", "--summary")]
    if bad:
        print(__doc__.split("\n\n")[-3], file=sys.stderr)
        return 2
    index = Index.of_cache() if cache else Index.of_repo()
    records = scan_documents(docs or None, index)
    if summary:
        counts = collections.Counter(p for c in records if not c.quote for p in projects(c))
        for p, n in sorted(counts.items(), key=lambda x: (-x[1], x[0])):
            print(f"{n}\t{p}")
        print(f"# {len(records)} citations, {len(counts)} projects")
        return 0
    print("\t".join(FIELDS))
    for c in records:
        print("\t".join(str(getattr(c, f)) for f in FIELDS))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
