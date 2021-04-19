mod model;
mod qml;

use cstr::cstr;
use qmetaobject::*;

pub fn main() {
    qmetaobject::log::init_qt_to_rust();
    qml::register();
    qml_register_type::<model::Game>(cstr!("Dombreaker.Game"), 1, 0, cstr!("Game"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/dombreaker/main.qml".into());
    engine.exec();
}
