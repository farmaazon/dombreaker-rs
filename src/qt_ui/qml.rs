use qmetaobject::*;

qrc! {qt_ui_qmls,
    "dombreaker" {
        "qml/main.qml" as "main.qml",
        "qml/Board.qml" as "Board.qml",
        "qml/Domino.qml" as "Domino.qml",
        "qml/DominoHalf.qml" as "DominoHalf.qml",
        "qml/Dot.qml" as "Dot.qml",
    }
}

pub fn register() {
    qt_ui_qmls();
}
