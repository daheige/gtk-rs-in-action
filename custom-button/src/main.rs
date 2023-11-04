mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "my.app.custom.button";

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
    let button = CustomButton::with_label("press me");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // connect to clicked signal of button
    button.connect_clicked(move |button| {
        button.set_label("hello world!");
    });

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        // .default_height(200)
        .default_width(200)
        .title("my gtk app")
        .child(&button)
        .build();

    window.present();
}
