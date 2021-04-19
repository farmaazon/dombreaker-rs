mod game;
mod qt_ui;

pub use log;

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    qt_ui::main();
}
