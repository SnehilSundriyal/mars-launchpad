mod apps;
mod ui;
mod utils;

use gtk::Application;
use gtk::prelude::*;

const APP_ID: &str = "com.snehil.marslaunchpad";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::launchpad::build_ui);

    app.run();
}
