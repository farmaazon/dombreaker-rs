import QtQuick 2.12
import QtQuick.Window 2.12
import Dombreaker.Game 1.0

Window {
    width: board.width
    height: board.height
    visible: true
    title: "Domino Breaker"

    Rectangle {
        id: board

        property string level: "
----------
|--------|
||------||
|||----|||
||||--||||
||||--||||
|||----|||
||------||
|--------|"
        property int tile_size: 40

        width: game.board_width * tile_size;
        height: game.board_height * tile_size;

        Game {
            id: game

            Component.onCompleted: {
                console.log(game.dominoes);
            }
        }

        Repeater {
            model: game.dominoes

            Text {
                x: domino.board_position.x * board.tile_size
                y: domino.board_position.y * board.tile_size
                text: "[" + domino.head_value + "|" + domino.tail_value + "]"


                Component.onCompleted: {
                    console.log(domino.board_position);
                }
            }
        }

        function recreate_game() {
            game.new_game(board.level)
        }

        Component.onCompleted: recreate_game()
        onLevelChanged: recreate_game()
    }
}
