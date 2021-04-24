mod model;
mod qml;

use cstr::cstr;
use qmetaobject::*;

pub fn main() {
    qmetaobject::log::init_qt_to_rust();
    qml::register();
    qml_register_type::<model::Game>(cstr!("Dombreaker.Models"), 1, 0, cstr!("GameModel"));
    qml_register_type::<model::Domino>(cstr!("Dombreaker.Models"), 1, 0, cstr!("DominoModel"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/dombreaker/main.qml".into());
    engine.exec();
}
