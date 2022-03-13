import QtQuick 2.0
import QtQuick.Window 2.2
import QtQuick.Layouts 1.3

Window {
    visible: true
    width: 300
    height: 500

    ColumnLayout {
        anchors.fill: parent
        MyItem {
            name: "ADN"
        }
        MyItem {
            name: "This"
        }
    }
}
