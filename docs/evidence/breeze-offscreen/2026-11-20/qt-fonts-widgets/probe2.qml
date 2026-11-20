import QtQuick
import org.kde.kirigami as Kirigami
Item {
    width: 400; height: 200
    Kirigami.Heading { objectName: "h1"; level: 1; text: "Hg"
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
    Kirigami.Heading { objectName: "h2"; level: 2; y: 40; text: "Hg"
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
    Text { objectName: "small"; y: 80; text: "Hg"; font: Kirigami.Theme.smallFont
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
}
