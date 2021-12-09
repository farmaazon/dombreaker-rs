mod game;

#[cfg(feature = "qml_ui")]
mod qt_ui;
#[cfg(feature = "sixtyfps_ui")]
mod sixty_ui;

pub use log;

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    cfg_if::cfg_if! {
        if #[cfg(feature = "sixtyfps_ui")] {
            sixty_ui::main();
        } else if #[cfg(feature = "qml_ui")] {
            qt_ui::main();
        } else {
            compile_error!("No UI framework enabled. Please run cargo with `--feature sixtyfps` or \
            `--feature qt` option");
        }
    }
}
