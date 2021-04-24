import QtQuick 2.12
import QtQuick.Layouts 1.2

Item {
    id: dominoHalf;
    property int value;
    height: width

    Grid {
        columns: 3

        anchors.fill: parent
        columnSpacing: width / 8
        rowSpacing: width / 8

        Dot {
            shown: value >= 4
        }

        Dot {
            shown: value >= 8
        }

        Dot {
            shown: value >= 2
        }

        Dot {
            shown: value >= 6
        }

        Dot {
            shown: value % 2 == 1
        }

        Dot {
            shown: value >= 6
        }

        Dot {
            shown: value >= 2
        }

        Dot {
            shown: value >= 8
        }

        Dot {
            shown: value >= 4
        }
    }
}
