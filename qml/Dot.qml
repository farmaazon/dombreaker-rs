import QtQuick 2.12

Item {
    property bool shown: false

    id: dot
    width: dominoHalf.width / 4;
    height: width;

    Rectangle {
        anchors.fill: parent
        radius: width/2.0;
        color: "black"
        visible: dot.shown
    }
}

