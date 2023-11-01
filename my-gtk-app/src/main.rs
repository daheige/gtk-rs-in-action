use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.my.HelloWorld";

fn build_ui(app: &Application) {
    // create a button
    let button = Button::builder()
        .label("press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // connect to clicked signal of button
    button.connect_clicked(move |button| {
        button.set_label("hello,world!");
    });

    // create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&button)
        .title("my gtk app")
        // .default_width(100)
        // .default_height(100)
        .build();

    // present window
    window.present();
}

fn main() -> glib::ExitCode {
    println!("Hello, world!");
    // create a new app use the builder pattern
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal of app.
    app.connect_activate(build_ui);

    // run app returns ExitCode
    app.run()
}
