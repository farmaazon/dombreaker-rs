import QtQuick 2.12
import QtQuick.Layouts 1.2

Rectangle {
    property point boardPosition
    property bool horizontal: false

    signal clicked();

    x: boardPosition.x * board.tileSize
    y: boardPosition.y * board.tileSize
    border.color: "green";
    border.width: 4;
    color: "red";
    width: board.tileSize;
    height: 2*board.tileSize;
    transform: Rotation {
        origin.x: board.tileSize/2.0;
        origin.y: board.tileSize/2.0;
        angle: horizontal ? -90 : 0;
    }

    MouseArea {
        id: mouse
        anchors.fill: parent
    }

    Component.onCompleted: mouse.clicked.connect(clicked)
}
