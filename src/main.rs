use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("totalumbrella.todolist")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("simple todo")
            .default_width(800)
            .default_height(600)
            .build();

        window.show();
    });

    app.run();
}
