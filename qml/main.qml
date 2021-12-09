import QtQuick 2.12
import QtQuick.Window 2.12
import QtQuick.Controls 2.15
import Dombreaker.Models 1.0

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

    Button {
        anchors.centerIn: parent
        text: "Try again!"
        visible: game.finished

        onClicked: recreate_game()
    }


    property string level: "--------
|------|
||----||
|||--|||
|||--|||
||----||
|------|".repeat(100)

    GameModel {
        id: game
    }

    function recreate_game() {
        game.new_game(level)
    }

    Component.onCompleted: recreate_game()
    onLevelChanged: recreate_game()
}
