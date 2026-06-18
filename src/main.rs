mod apps;

use apps::discover_apps;

use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    FlowBox,
    Label,
    ScrolledWindow,
};

const APP_ID: &str = "com.snehil.marslaunchpad";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let apps = discover_apps();

    let flow = FlowBox::builder()
        .max_children_per_line(6)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    for desktop_app in apps {
        let label = Label::new(Some(&desktop_app.name));

        label.set_width_chars(20);
        label.set_wrap(true);

        flow.insert(&label, -1);
    }

    let scroll = ScrolledWindow::builder()
        .child(&flow)
        .vexpand(true)
        .hexpand(true)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mars Launchpad")
        .default_width(1400)
        .default_height(900)
        .child(&scroll)
        .build();

    window.present();
}