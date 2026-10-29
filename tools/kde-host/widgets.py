#!/usr/bin/env python3
"""The Qt Widgets host of the KDE harness (the 0.8.0 plan, D4; spike K).

A QPushButton of the Breeze style in each state, every one in a cell of the
palette's window background, each cell grabbed (QWidget.grab) to
<out>/<state>.png, with <out>/index.toml recording the toolkit's versions, the
palette, the font and each button's style option state. Run by
tools/kde-host/run.sh with the container's own PySide6, which uses the
container's Qt and so loads its Breeze style plugin:

    python3 tools/kde-host/widgets.py <out>

The states: normal; hovered (the button marked as under the mouse,
Qt::WA_UnderMouse, and sent a mouse move over its centre, which a button with
mouse tracking needs; QStyleOption reads State_MouseOver from it); pressed
(setDown); disabled; checked (checkable, setChecked). The window holds the
keyboard focus, so no button has it. The cells' margins are the layout's,
which the style states.
"""

import sys
from pathlib import Path

import PySide6
from PySide6.QtCore import QEvent, QPointF, Qt, QTimer, qVersion
from PySide6.QtGui import QColor, QGuiApplication, QMouseEvent, QPalette
from PySide6.QtWidgets import QApplication, QHBoxLayout, QPushButton, QStyle, QStyleOptionButton, QWidget

STATES = ("normal", "hovered", "pressed", "disabled", "checked")
ROLES = ("Window", "WindowText", "Button", "ButtonText", "Highlight", "HighlightedText")
GROUPS = ("Active", "Inactive", "Disabled")


class Button(QPushButton):
    def state_flags(self):
        option = QStyleOptionButton()
        self.initStyleOption(option)
        return [flag.name for flag in QStyle.StateFlag if flag.value and option.state & flag]


def toml(value):
    return '"' + str(value).replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    if len(sys.argv) != 2:
        print("usage: widgets.py <out>", file=sys.stderr)
        return 2
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)

    app = QApplication(sys.argv[:1])
    style = app.setStyle("Breeze")
    if style is None or app.style().name().lower() != "breeze":
        print(f"the Breeze style did not load (style: {app.style().name()})", file=sys.stderr)
        return 1

    window = QWidget()
    layout = QHBoxLayout(window)
    layout.setContentsMargins(0, 0, 0, 0)
    layout.setSpacing(0)
    cells = {}
    for state in STATES:
        cell = QWidget()
        cell.setAutoFillBackground(True)
        button = Button("Button")
        QHBoxLayout(cell).addWidget(button)
        layout.addWidget(cell)
        cells[state] = (cell, button)
    cells["pressed"][1].setDown(True)
    cells["disabled"][1].setEnabled(False)
    cells["checked"][1].setCheckable(True)
    cells["checked"][1].setChecked(True)

    window.move(100, 100)
    window.show()
    window.activateWindow()
    status = []

    def grab():
        window.setFocus()
        hovered = cells["hovered"][1]
        hovered.setAttribute(Qt.WidgetAttribute.WA_UnderMouse, True)
        centre = QPointF(hovered.rect().center())
        QApplication.sendEvent(
            hovered,
            QMouseEvent(QEvent.Type.MouseMove, centre, hovered.mapToGlobal(centre),
                        Qt.MouseButton.NoButton, Qt.MouseButton.NoButton, Qt.KeyboardModifier.NoModifier),
        )
        palette = app.palette()
        lines = [
            "# Written by tools/kde-host/widgets.py.",
            "",
            "[host]",
            f"qt = {toml(qVersion())}",
            f"pyside6 = {toml(PySide6.__version__)}",
            f"platform = {toml(QGuiApplication.platformName())}",
            f"style = {toml(app.style().name())}",
            f"font = {toml(app.font().toString())}",
            f"device_pixel_ratio = {window.devicePixelRatioF()}",
            f"window_active = {str(window.isActiveWindow()).lower()}",
            "",
            "[palette]",
        ]
        for group in GROUPS:
            for role in ROLES:
                colour = palette.color(getattr(QPalette.ColorGroup, group), getattr(QPalette.ColorRole, role))
                lines.append(f"{group}.{role} = {toml(colour.name(QColor.NameFormat.HexArgb))}")
        for state in STATES:
            cell, button = cells[state]
            path = out / f"{state}.png"
            image = cell.grab()
            if not image.save(str(path)):
                print(f"cannot write {path}", file=sys.stderr)
                status.append(1)
            lines += [
                "",
                "[[cell]]",
                f"state = {toml(state)}",
                f"png = {toml(path.name)}",
                f"size = [{image.width()}, {image.height()}]",
                f"button = [{button.x()}, {button.y()}, {button.width()}, {button.height()}]",
                "style_state = [" + ", ".join(toml(name) for name in button.state_flags()) + "]",
            ]
        (out / "index.toml").write_text("\n".join(lines) + "\n")
        app.exit(1 if status else 0)

    QTimer.singleShot(1000, grab)
    return app.exec()


if __name__ == "__main__":
    sys.exit(main())
