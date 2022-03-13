/*
 We don't need these right now
extern crate qt_core;
pub use qt_core::{q_init_resource, qs};
extern crate qt_gui;
pub use qt_gui::QGuiApplication;
extern crate qt_qml;
pub use qt_qml::QQmlApplicationEngine;
*/

extern crate ce_usb_protocol;
use ce_usb_protocol::discover::get_calc_dev;

fn main() {
    let calc_dev = get_calc_dev();

    // Start up QML machine
    // Currently, because we have to fix the backend/API first
    /*
    QGuiApplication::init(|_| unsafe {
        q_init_resource!("resources");
        let _engine = QQmlApplicationEngine::from_q_string(&qs("qrc:/main.qml"));
        QGuiApplication::exec()
    })
    */
}
