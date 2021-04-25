import QtQuick 2.12
import Dombreaker.Models 1.0

Rectangle {
    id: board

    property int tileSize: 80
    property GameModel game;

    width: game.board_width * tileSize;
    height: game.board_height * tileSize;


    Component {
        id: brokenDominoComponent
        Domino {
            id: brokenDomino
            PropertyAnimation on opacity {
                from: 1.0
                to: 0.0
                duration: 1500.0
                easing.type: Easing.InSine

                onFinished: brokenDomino.destroy()
            }
        }

    }

    Repeater {
        model: game.dominoes

        DominoBackground {
            horizontal: domino.horizontal
            boardPosition: domino.board_position

            onClicked: {
                game.domino_hit(domino.game_id)
            }

            Connections {
                target: domino

                function onBroken() {
                    let properties = {
                        horizontal: domino.horizontal,
                        boardPosition: domino.board_position,
                        headValue: domino.head_value,
                        tailValue: domino.tail_value,
                    }
                    if (brokenDominoComponent.createObject(board, properties) === null) {
                        console.error("ERROR while creating broken Domino")
                    }
                }
            }
        }
    }
}
