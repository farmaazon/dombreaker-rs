fn main() {
    #[cfg(feature = "sixtyfps_ui")]
    sixtyfps_build::compile("sixty/Main.60").unwrap();
}
