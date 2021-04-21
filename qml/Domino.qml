import QtQuick 2.12
import QtQuick.Layouts 1.2

Rectangle {
    property int tileSize;
    property int headValue;
    property int tailValue;
    property bool horizontal;

    signal isHit();

    border.color: "green";
    border.width: 4;
    color: "red";
    width: tileSize;
    height: 2*tileSize;
    transform: Rotation {
        origin.x: tileSize/2.0;
        origin.y: tileSize/2.0;
        angle: horizontal ? -90 : 0;
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 3

        DominoHalf {
            width: 64 //TODO why parent.width does not work?
            value: headValue
            Component.onCompleted: console.log("parent", parent.width)
        }

        Rectangle {
            width: parent.width
            height: 2
            color: "darkred"
        }

        DominoHalf {
            width: 64 //TODO why parent.width does not work?
            value: tailValue
        }

        Component.onCompleted: console.log(width)
    }

    MouseArea {
        id: mouse
        anchors.fill: parent
    }

    Component.onCompleted: mouse.clicked.connect(isHit)
}
