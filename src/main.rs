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
use glib::clone;

fn main() {
    let application = Application::builder()
        .application_id("com.andrewradev.ClickExample")
        .build();

    application.connect_activate(|app| build_ui(app));
    application.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Click Example")
        .default_width(800)
        .default_height(400)
        .build();

    let outer_container = GtkBox::new(Orientation::Vertical, 10);
    let button = Button::with_label("Add click entry");

    // GTK_DEBUG=interactive
    // let scrolled_window = gtk::ScrolledWindow::new();

    let scrolled_window = ScrolledWindowBuilder::new().
        vexpand(true).
        build();

    let entry_container = GtkBox::new(Orientation::Vertical, 10);

    // button.connect_clicked(move |_| {
    //     let info = format!("Clicked on the button at {}", ::chrono::Local::now());
    //     let label = Label::new(Some(&info));
    //
    //     entry_container.append(&label);
    // });

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
