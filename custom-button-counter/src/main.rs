mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "my.app.custom.button.couter";

fn main() -> glib::ExitCode {
    println!("Hello, world!");
    // create app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app: &Application) {
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        // .default_height(200)
        .default_width(200)
        .title("custom-counter")
        .child(&button)
        .build();

    window.present();
}
