mod rustpad_func;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry};

const APP_ID: &str = "org.gtk_rs.Rustpad";

fn build_ui(app: &Application) {
    let window = main_window(app);
    let main_widget = gtk::Box::new(
        gtk::Orientation::Vertical, 0);

    let menubar = build_menubar();

    let textbox = Entry::new();
    textbox.set_text("Hello, world!");

    main_widget.append(&menubar);
    main_widget.append(&textbox);

    window.set_child(Some(&main_widget));
    window.present();
}

fn build_menubar() -> gtk::Box{
    let menu_items = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let save_file_btn = gtk::Button::with_label("Save File...");
    let load_file_btn = gtk::Button::with_label("Load File...");

    save_file_btn.connect_clicked(move | _button| {
        println!("Save File...");
        rustpad_func::save_file();
    });
    load_file_btn.connect_clicked(move | _button| {
        println!("Load File...");
        rustpad_func::load_file();
    });

    menu_items.append(&save_file_btn);
    menu_items.append(&load_file_btn);

    return menu_items;
}

fn main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
    .application(app)
    .default_width(320)
    .default_height(200)
    .title("Rustpad")
    .build();

    return window;
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
