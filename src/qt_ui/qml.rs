use qmetaobject::*;

qrc! {qt_ui_qmls,
    "dombreaker" {
        "qml/main.qml" as "main.qml",
    }
}

pub fn register() {
    qt_ui_qmls();
}
