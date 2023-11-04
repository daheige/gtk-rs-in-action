use std::rc::Rc;

use gtk::{
    glib::{self, clone},
    prelude::*,
};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("my.examples.dialog")
        .build();

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let button = gtk::Button::builder()
        .label("Open Dialog")
        .halign(gtk::Align::Center) // 居中对齐
        .valign(gtk::Align::Center)
        .build();

    // 引用计数的方式声明window
    let window = Rc::new(
        gtk::ApplicationWindow::builder()
            .application(application)
            .title("dialog example")
            .default_width(350)
            .default_height(70)
            .child(&button)
            .visible(true)
            .build(),
    );

    button.connect_clicked(clone!(@strong window =>
        move |_| {
            gtk::glib::MainContext::default().spawn_local(dialog(Rc::clone(&window)));
        }
    ));

    // 关闭操作绑定
    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}

async fn dialog<W: IsA<gtk::Window>>(window: Rc<W>) {
    let question_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .buttons(["Cancel", "Ok"])
        .message("What is your answer?")
        .build();

    let answer = question_dialog.choose_future(Some(&*window)).await;
    let info_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .message("You answered")
        .detail(format!("Your answer: {:?}", answer))
        .build();

    info_dialog.show(Some(&*window));
}
