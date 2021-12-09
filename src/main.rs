mod game;

#[cfg(feature = "qmetaobject")]
mod qt_ui;

pub use log;

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    #[cfg(feature = "qmetaobject")]
    qt_ui::main();
}
