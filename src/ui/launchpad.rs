use crate::apps::discover_apps;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::SearchEntry;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label, Orientation, ScrolledWindow};
const APPS_PER_PAGE: usize = 24;

pub fn build_ui(app: &Application) {
    load_css();

    let all_apps = Rc::new(discover_apps());

    let pages = Rc::new(
        all_apps
            .chunks(APPS_PER_PAGE)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    );

    println!("Pages: {}", pages.len());
    let current_page = Rc::new(RefCell::new(0usize));
    let search = SearchEntry::new();

    search.set_placeholder_text(Some("Search Applications"));
    search.set_width_chars(40);
    search.set_halign(gtk::Align::Center);

    let page_label = Label::new(None);

    update_page_indicator(&page_label, 0, pages.len());


    let nav_row = gtk::Box::new(Orientation::Horizontal, 24);

    let root = gtk::Box::new(Orientation::Vertical, 16);

    nav_row.set_halign(gtk::Align::Center);

    nav_row.append(&page_label);


    let grid = Rc::new(Grid::new());

    grid.set_column_spacing(64);
    grid.set_row_spacing(48);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);
    
    let flow_search = grid.clone();
    let all_apps_search = all_apps.clone();

    let pages_search = pages.clone();
    let page_label_search = page_label.clone();
    let current_page_search = current_page.clone();

    search.connect_search_changed(move |entry| {
        let query = entry
            .text()
            .to_string()
            .to_lowercase();

        if query.is_empty() {
            let page = *current_page_search.borrow();
            page_label_search.set_visible(true);
            render_apps(
                flow_search.as_ref(),
                &pages_search[page],
            );

            update_page_indicator(
                &page_label_search,
                page,
                pages_search.len(),
            );

            return;
        }

        let filtered: Vec<_> = all_apps_search
            .iter()
            .filter(|app| {
                app.name
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect();

        page_label_search.set_visible(false);

        render_apps(
            flow_search.as_ref(),
            &filtered,
        );
    });

    let scroll_controller =
        gtk::EventControllerScroll::new(
            gtk::EventControllerScrollFlags::VERTICAL,
        );

    let flow_scroll = grid.clone();
    let pages_scroll = pages.clone();
    let page_label_scroll = page_label.clone();
    let current_page_scroll = current_page.clone();

    scroll_controller.connect_scroll(
        move |_, _, dy| {
            let mut page =
                current_page_scroll.borrow_mut();

            if dy > 0.0 {
                if *page + 1 < pages_scroll.len() {
                    *page += 1;
                }
            } else if dy < 0.0 {
                if *page > 0 {
                    *page -= 1;
                }
            }

            render_apps(
                flow_scroll.as_ref(),
                &pages_scroll[*page],
            );

            update_page_indicator(
                &page_label_scroll,
                *page,
                pages_scroll.len(),
            );

            gtk::glib::Propagation::Stop
        },
    );

    render_apps(
        grid.as_ref(),
        &pages[0],
    );

    let scroll = ScrolledWindow::builder()
        .child(grid.as_ref())
        .vexpand(true)
        .hexpand(true)
        .build();

    root.append(&search);
    root.append(&scroll);
    root.append(&nav_row);


    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mars Launchpad")
        .default_width(1600)
        .default_height(900)
        .child(&root)
        .build();

    window.set_opacity(0.85);
    window.add_controller(scroll_controller);
    window.fullscreen();
    window.present();
}

fn render_apps(
    grid: &gtk::Grid,
    apps: &[crate::apps::DesktopApp],
) {
    while let Some(child) = grid.first_child() {
        grid.remove(&child);
    }

    for (i, desktop_app) in apps.iter().enumerate() {
        let exec = desktop_app.exec.clone();

        let tile = gtk::Box::new(
            gtk::Orientation::Vertical,
            8,
        );

        tile.set_size_request(160, 160);

        let icon_name = if desktop_app.icon.is_empty() {
            "application-x-executable"
        } else {
            &desktop_app.icon
        };

        let icon = gtk::Image::from_icon_name(icon_name);
        icon.set_pixel_size(96);

        let label =
            gtk::Label::new(Some(&desktop_app.name));

        label.set_wrap(true);
        label.set_max_width_chars(12);
        label.set_justify(
            gtk::Justification::Center,
        );

        tile.append(&icon);
        tile.append(&label);

        let button = gtk::Button::new();

        button.set_child(Some(&tile));

        button.connect_clicked(move |_| {
            crate::utils::launch_app(&exec);
        });

        let row = (i / 8) as i32;
        let col = (i % 8) as i32;

        grid.attach(
            &button,
            col,
            row,
            1,
            1,
        );
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

fn load_css() {
    let provider = gtk::CssProvider::new();

    provider.load_from_data(
        "
        window {
            background: rgba(0, 0, 0, 0.35);
        }

        button {
            background: transparent;
            border: none;
            box-shadow: none;
        }

        button:hover {
        background: transparent;
        border: none;
        box-shadow: none;
        }

        button:active {
        background: transparent;
        border: none;
        box-shadow: none;
        }

        button.flat {
        background: transparent;
        }

        searchentry {
            border-radius: 20px;
            padding: 8px;
            background: rgba(255,255,255,0.12);
        }

        searchentry text {
            color: white;
        }
        "
    );

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
