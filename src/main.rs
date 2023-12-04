use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Rustpad")
        .build();

    let textbox = Entry::new();
    textbox.set_text("Hello, world!");

    window.set_child(Some(&textbox));
    window.present();
}
