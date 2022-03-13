fn main() {
    qt_ritual_build::add_resources(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/resources.qrc"));
    // I don't actually know what this does
}
