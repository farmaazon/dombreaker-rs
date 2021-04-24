import QtQuick 2.12
import QtQuick.Layouts 1.2

DominoBackground {
    property int headValue;
    property int tailValue;

    Column {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 7

        DominoHalf {
            width: parent.width
            value: headValue
        }

        Rectangle {
            width: parent.width
            height: 2
            color: "darkred"
        }

        DominoHalf {
            width: parent.width
            value: tailValue
        }
    }
}
