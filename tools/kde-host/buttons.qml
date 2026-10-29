// The QML host of the KDE harness (the 0.8.0 plan, D4; spike K): a Button of
// the org.kde.desktop style (qqc2-desktop-style, which draws through the
// application's QStyle, Breeze) in each state, every one in a cell of the
// Kirigami theme's window background, each cell grabbed to <out>/<state>.png.
// Run by tools/kde-host/run.sh with the `qml` runtime as a widget application
// (the style's StyleItem needs a QApplication):
//
//     qml --apptype widget tools/kde-host/buttons.qml -- <out>
//
// The states: normal; hovered (Button.hovered is read-only and follows the
// pointer, so the style's background item, a StyleItem whose `hover` the style
// binds to it, is set directly); pressed (`down`, which the style draws as
// sunken); disabled; checked (checkable, `checked`). Each line printed with the
// prefix "index:" records a cell or the theme, for run.sh's index.txt.

import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.ApplicationWindow {
    id: root

    readonly property string outDir: {
        const args = Qt.application.arguments;
        const i = args.indexOf("--");
        return i >= 0 && i + 1 < args.length ? args[i + 1] : "";
    }
    readonly property var cells: [normal, hovered, pressed, disabled, checked]
    property int next: 0

    function grabNext() {
        if (next >= cells.length) {
            console.info(`index: window active=${root.active}`);
            Qt.quit();
            return;
        }
        const cell = cells[next];
        cell.grabToImage(result => {
            const path = `${root.outDir}/${cell.name}.png`;
            if (!result.saveToFile(path)) {
                console.info(`index: error cannot write ${path}`);
                Qt.exit(3);
                return;
            }
            const b = cell.button;
            console.info(`index: cell ${cell.name} size=${cell.width}x${cell.height} button=${b.x},${b.y},${b.width}x${b.height} down=${b.down} hovered=${b.hovered} background.hover=${b.background.hover} background.sunken=${b.background.sunken} enabled=${b.enabled} checked=${b.checked} activeFocus=${b.activeFocus}`);
            root.next += 1;
            root.grabNext();
        });
    }

    component Cell: Rectangle {
        property string name
        property alias button: button

        width: button.implicitWidth + 2 * Kirigami.Units.largeSpacing
        height: button.implicitHeight + 2 * Kirigami.Units.largeSpacing
        color: Kirigami.Theme.backgroundColor

        QQC2.Button {
            id: button
            x: Kirigami.Units.largeSpacing
            y: Kirigami.Units.largeSpacing
            text: "Button"
        }
    }

    x: 100
    y: 100
    width: row.implicitWidth
    height: row.implicitHeight
    visible: true
    color: Kirigami.Theme.backgroundColor

    Component.onCompleted: {
        if (outDir === "") {
            console.info("index: error no output folder (qml ... buttons.qml -- <out>)");
            Qt.exit(2);
            return;
        }
        requestActivate();
        console.info(`index: theme window background=${Kirigami.Theme.backgroundColor} text=${Kirigami.Theme.textColor} highlight=${Kirigami.Theme.highlightColor} focus=${Kirigami.Theme.focusColor} hover=${Kirigami.Theme.hoverColor}`);
        console.info(`index: theme button background=${normal.button.Kirigami.Theme.backgroundColor} text=${normal.button.Kirigami.Theme.textColor}`);
        console.info(`index: font ${normal.button.font.family} ${normal.button.font.pointSize}pt; largeSpacing=${Kirigami.Units.largeSpacing}; devicePixelRatio=${Screen.devicePixelRatio}`);
    }

    Row {
        id: row

        Cell {
            id: normal
            name: "normal"
        }
        Cell {
            id: hovered
            name: "hovered"
            Component.onCompleted: button.background.hover = true
        }
        Cell {
            id: pressed
            name: "pressed"
            button.down: true
        }
        Cell {
            id: disabled
            name: "disabled"
            button.enabled: false
        }
        Cell {
            id: checked
            name: "checked"
            button.checkable: true
            button.checked: true
        }
    }

    Timer {
        interval: 1000
        running: true
        onTriggered: root.grabNext()
    }
}
