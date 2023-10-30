# gtk-rs
GTK 4 is the newest version of a popular cross-platform widget toolkit written in C. Thanks to GObject-Introspection, GTK's API can be easily targeted by various programming languages.  The API even describes the ownership of its parameters!
https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html

# install gtk4
mac os
```shell
brew install gtk4
```

linux
Fedora and derivatives:
```shell
sudo dnf install gtk4-devel gcc
```

Debian and derivatives:
```shell
sudo apt install libgtk-4-dev build-essential
```
Arch and derivatives:
```shell
sudo pacman -S gtk4 base-devel
```

# project setup
https://gtk-rs.org/gtk4-rs/stable/latest/book/project_setup.html
```shell
cd ~
cargo new my-gtk-app
% pkg-config --modversion gtk4
4.12.3
cd my-gtk-app
cargo add gtk4 --rename gtk --features v4_10
cargo run
```

# hello world
https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
```rust
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
    button.connect_clicked(|button| {
        button.set_label("hello,world!");
    });

    // create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&button)
        .title("my gtk app")
        .default_width(100)
        .default_height(100)
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
```
cargo run
![](hello-world.jpg)
