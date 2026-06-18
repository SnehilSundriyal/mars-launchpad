use crate::apps::discover_apps;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FlowBox, Label, Orientation, ScrolledWindow};
const APPS_PER_PAGE: usize = 24;

pub fn build_ui(app: &Application) {
    let apps = discover_apps();

    let pages = Rc::new(
        apps.chunks(APPS_PER_PAGE)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    );

    println!("Pages: {}", pages.len());
    let current_page = Rc::new(RefCell::new(0usize));
    let page_label = Label::new(None);

    update_page_indicator(&page_label, 0, pages.len());
    let prev_button = Button::with_label("←");
    let next_button = Button::with_label("→");

    let nav_row = gtk::Box::new(Orientation::Horizontal, 24);

    let root = gtk::Box::new(Orientation::Vertical, 16);

    root.append(&nav_row);

    nav_row.set_halign(gtk::Align::Center);

    nav_row.append(&prev_button);
    nav_row.append(&page_label);
    nav_row.append(&next_button);


    let flow = Rc::new(
        FlowBox::builder()
            .max_children_per_line(8)
            .selection_mode(gtk::SelectionMode::None)
            .column_spacing(24)
            .row_spacing(24)
            .build()
    );

    let flow_next = flow.clone();
    let pages_next = pages.clone();
    let page_label_next = page_label.clone();
    let current_page_next = current_page.clone();

    next_button.connect_clicked(move |_| {
        let mut page = current_page_next.borrow_mut();

        if *page + 1 < pages_next.len() {
            *page += 1;

            render_page(
                flow_next.as_ref(),
                &pages_next[*page],
            );

            update_page_indicator(
                &page_label_next,
                *page,
                pages_next.len(),
            );
        }
    });

    let flow_prev = flow.clone();
    let pages_prev = pages.clone();
    let page_label_prev = page_label.clone();
    let current_page_prev = current_page.clone();

    prev_button.connect_clicked(move |_| {
        let mut page = current_page_prev.borrow_mut();

        if *page > 0 {
            *page -= 1;

            render_page(
                flow_prev.as_ref(),
                &pages_prev[*page],
            );

            update_page_indicator(
                &page_label_prev,
                *page,
                pages_prev.len(),
            );
        }
    });

    render_page(flow.as_ref(), &pages[0]);

    let scroll = ScrolledWindow::builder()
        .child(flow.as_ref())
        .vexpand(true)
        .hexpand(true)
        .build();

    root.append(&scroll);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mars Launchpad")
        .default_width(1600)
        .default_height(900)
        .child(&root)
        .build();

    window.present();
}

fn render_page(flow: &gtk::FlowBox, page: &[crate::apps::DesktopApp]) {
    while let Some(child) = flow.first_child() {
        flow.remove(&child);
    }

    for desktop_app in page {
        let exec = desktop_app.exec.clone();

        let tile = gtk::Box::new(gtk::Orientation::Vertical, 8);

        tile.set_size_request(160, 160);

        let icon_name = if desktop_app.icon.is_empty() {
            "application-x-executable"
        } else {
            &desktop_app.icon
        };

        let icon = gtk::Image::from_icon_name(icon_name);
        icon.set_pixel_size(80);

        let label = gtk::Label::new(Some(&desktop_app.name));

        label.set_wrap(true);
        label.set_max_width_chars(12);
        label.set_justify(gtk::Justification::Center);

        tile.append(&icon);
        tile.append(&label);

        let button = gtk::Button::new();

        button.set_child(Some(&tile));

        button.connect_clicked(move |_| {
            crate::utils::launch_app(&exec);
        });

        flow.insert(&button, -1);
    }
}

fn update_page_indicator(label: &gtk::Label, current: usize, total: usize) {
    let mut dots = String::new();

    for i in 0..total {
        if i == current {
            dots.push('●');
        } else {
            dots.push('○');
        }

        dots.push(' ');
    }

    label.set_text(dots.trim());
}
