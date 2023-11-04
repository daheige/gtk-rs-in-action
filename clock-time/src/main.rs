use chrono::Local;
use gtk::{glib, prelude::*,Label, Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    let app = Application::builder()
        .application_id("my.gtk.clock-timer")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

// 实现clock
fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("current time"));
    window.set_default_size(260,60); // 设置宽度和高度

    let time = current_time(); // 获取系统时间
    let label = Label::default();
    label.set_text(&time);

    // 将label添加到窗口
    window.set_child(Some(&label));

    window.present();

    // 实现时间动态功能
    let ticker = move ||{
        let time = current_time();
        label.set_text(&time);

        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // 每秒触发一次
    glib::timeout_add_seconds_local(1,ticker);
}

fn current_time() -> String{
    let now = Local::now();
    format!("{}",now.format("%Y-%m-%d %H:%M:%S"))
}
