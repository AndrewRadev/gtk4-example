use std::env;
use std::ffi::OsString;

// use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Button,
    Label,
    Box as GtkBox,
    Orientation,
    ScrolledWindowBuilder,
};
use gio::ApplicationFlags;
use glib::clone;

fn main() {
    let application = Application::builder().
        application_id("com.andrewradev.ClickExample").
        flags(ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE).
        build();

    application.connect_command_line(move |app, cmdline| {
        build_ui(&app, cmdline.arguments());
        0
    });

    application.run_with_args(&env::args().collect::<Vec<_>>());
}

fn build_ui(app: &Application, arguments: Vec<OsString>) {
    let button_label = arguments.get(1).
        and_then(|arg| arg.clone().into_string().ok()).
        unwrap_or_else(|| String::from("Add click entry"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Click Example")
        .default_width(800)
        .default_height(400)
        .build();

    let outer_container = GtkBox::new(Orientation::Vertical, 10);
    let button = Button::with_label(&button_label);
    let scrolled_window = ScrolledWindowBuilder::new().
        vexpand(true).
        build();
    let entry_container = GtkBox::new(Orientation::Vertical, 10);

    button.connect_clicked(clone!(@strong entry_container => move |_| {
        let info = format!("Clicked on the button at {}", ::chrono::Local::now());
        let label = Label::new(Some(&info));

        entry_container.append(&label);
    }));

    outer_container.append(&button);
    scrolled_window.set_child(Some(&entry_container));
    outer_container.append(&scrolled_window);

    window.set_child(Some(&outer_container));
    window.show();
}
