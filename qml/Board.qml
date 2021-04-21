import QtQuick 2.12
import Dombreaker.Game 1.0

Rectangle {
    id: board

    property int tileSize: 80
    property Game game;

    width: game.board_width * tileSize;
    height: game.board_height * tileSize;

    Repeater {
        model: game.dominoes

        Domino {
            x: domino.board_position.x * board.tileSize
            y: domino.board_position.y * board.tileSize
            tileSize: board.tileSize
            headValue: domino.head_value
            tailValue: domino.tail_value
            horizontal: domino.horizontal

            onIsHit: {
                game.domino_hit(domino.game_id)
            }
        }
    }
}
