import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
Item {
    width: 400; height: 200
    Text { objectName: "plain"; text: "OK Hg"; font.pointSize: 10; font.family: "Noto Sans"
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
    QQC2.Label { objectName: "label"; y: 40; text: "OK Hg"
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
    Text { objectName: "kirigamiDefault"; y: 80; text: "OK Hg"; font: Kirigami.Theme.defaultFont
        property int infoPx: fontInfo.pixelSize; property real infoPt: fontInfo.pointSize }
    QQC2.Button { objectName: "button"; y: 120; text: "OK" }
}
