import QtQuick 2.12
import QtQuick.Window 2.12
import Dombreaker.Game 1.0

Window {
    width: board.width
    height: board.height + topBar.height
    visible: true
    title: "Domino Breaker"

    Column {
        Text {
            id: topBar
            text: "Score: " + game.score;
        }

        Board {
            id: board
            game: game
        }
    }


    property string level: "--------
|------|
||----||
|||--|||
|||--|||
||----||
|------|"

    Game {
        id: game
    }

    function recreate_game() {
        game.new_game(level)
    }

    Component.onCompleted: recreate_game()
    onLevelChanged: recreate_game()
}
