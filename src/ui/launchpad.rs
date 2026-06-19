use crate::apps::discover_apps;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::SearchEntry;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label, Orientation};
const APPS_PER_PAGE: usize = 24;

pub fn build_ui(app: &Application) {
    load_css();

    let all_apps = Rc::new(discover_apps());
    let current_results =
        Rc::new(RefCell::new(Vec::<crate::apps::DesktopApp>::new()));

    let pages = Rc::new(
        all_apps
            .chunks(APPS_PER_PAGE)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    );

    if let Some(first_page) = pages.first() {
        *current_results.borrow_mut() =
            first_page.clone();
    }

    let current_page = Rc::new(RefCell::new(0usize));
    let search = SearchEntry::new();

    search.set_placeholder_text(Some("Search Applications"));
    search.set_halign(gtk::Align::Center);

    let page_label = Label::new(None);
    page_label.add_css_class("page-dots");

    update_page_indicator(&page_label, 0, pages.len());


    let nav_row = gtk::Box::new(Orientation::Horizontal, 24);

    let root = gtk::Box::new(Orientation::Vertical, 8);
    root.set_valign(gtk::Align::Center);
    root.set_vexpand(true);
    root.set_spacing(28);

    nav_row.set_halign(gtk::Align::Center);

    nav_row.append(&page_label);
    nav_row.set_margin_top(16);
    nav_row.set_margin_bottom(24);


    let grid = Rc::new(Grid::new());

    grid.set_column_spacing(16);
    grid.set_row_spacing(12);
    grid.set_halign(gtk::Align::Center);
    grid.set_hexpand(true);
    root.set_halign(gtk::Align::Fill);
    root.set_hexpand(true);
    
    let grid_search = grid.clone();
    let all_apps_search = all_apps.clone();

    let pages_search = pages.clone();
    let page_label_search = page_label.clone();
    let current_page_search = current_page.clone();

    let current_results_search =
        current_results.clone();

    search.connect_search_changed(move |entry| {
        let query = entry
            .text()
            .to_string()
            .to_lowercase();

        if query.is_empty() {
            let page = *current_page_search.borrow();

            page_label_search.set_visible(true);

            *current_results_search.borrow_mut() =
                pages_search[page].clone();

            render_apps(
                grid_search.as_ref(),
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

        *current_results_search.borrow_mut() =
            filtered.clone();

        render_apps(
            grid_search.as_ref(),
            &filtered,
        );
    });

    let scroll_controller =
        gtk::EventControllerScroll::new(
            gtk::EventControllerScrollFlags::VERTICAL,
        );

    let grid_scroll = grid.clone();
    let pages_scroll = pages.clone();
    let page_label_scroll = page_label.clone();
    let current_page_scroll = current_page.clone();

    scroll_controller.connect_scroll(
        move |_, _, dy| {

            if !page_label_scroll.is_visible() {
                return gtk::glib::Propagation::Stop;
            }
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
                grid_scroll.as_ref(),
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

    grid.set_hexpand(true);
    grid.set_vexpand(true);

    let spacer = gtk::Box::new(Orientation::Vertical, 0);
    spacer.set_size_request(-1, 40);
    root.append(&search);
    root.append(&spacer);   // add this
    root.append(grid.as_ref());
    root.append(&nav_row);


    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mars Launchpad")
        .default_width(1600)
        .default_height(900)
        .child(&root)
        .build();

    let app_for_enter = app.clone();

    let key_controller = gtk::EventControllerKey::new();

    let results_for_enter =
        current_results.clone();

    search.connect_activate(move |_| {
        let results =
            results_for_enter.borrow();

        if let Some(app_entry) = results.first() {
            crate::utils::launch_app(
                &app_entry.exec,
            );

            app_for_enter.quit();
        }
    });

    let app_for_escape = app.clone();

    key_controller.connect_key_pressed(
        move |_, key, _, _| {

            if key == gtk::gdk::Key::Escape {
                app_for_escape.quit();

                return gtk::glib::Propagation::Stop;
            }

            gtk::glib::Propagation::Proceed
        },
    );

    search.add_controller(key_controller);

    window.set_opacity(0.85);
    window.add_controller(scroll_controller);
    window.fullscreen();
    window.present();

    search.grab_focus();
    search.select_region(0, -1);
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
            4,
        );

        tile.set_size_request(100, 100);

        let icon_name = if desktop_app.icon.is_empty() {
            "application-x-executable"
        } else {
            &desktop_app.icon
        };

        let icon = create_icon(icon_name);

        let label =
            gtk::Label::new(Some(&desktop_app.name));

        label.set_wrap(true);
        label.set_max_width_chars(10);   // was 12
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);  // add this
        label.set_wrap(false);           // turn off wrap, use ellipsis instead
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
            dots.push('⬤');   // filled — was ●
        } else {
            dots.push('○');   // open circle — was ·
        }
        dots.push(' ');
    }
    label.set_text(dots.trim());
}

fn create_icon(icon_name: &str) -> gtk::Image {
    use std::path::Path;

    if Path::new(icon_name).exists() {
        let image = gtk::Image::from_file(icon_name);
        image.set_pixel_size(72);
        return image;
    }

    let image = gtk::Image::from_icon_name(icon_name);

    image.set_pixel_size(72);

    image
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
    border-radius: 20px;
    transition: background 0.15s ease;
}

button:hover {
    background: rgba(255, 255, 255, 0.12);  /* soft white glow */
    border: none;
    box-shadow: none;
}

button:active {
    background: rgba(255, 255, 255, 0.22);
    border: none;
    box-shadow: none;
}

        button.flat {
        background: transparent;
        }

        searchentry {
    min-width: 280px;
    min-height: 34px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.18);
    border: 1px solid rgba(255, 255, 255, 0.28);
    padding: 0 14px;
    box-shadow: none;
    outline: none;
}

searchentry:focus {
    border-color: rgba(255, 255, 255, 0.5);
    background: rgba(255, 255, 255, 0.22);
    box-shadow: none;
    outline: none;
}

/* Kill GTK's default focus indicator */
searchentry > text {
    box-shadow: none;
}


        .page-dots {
            color: rgba(255, 255, 255, 0.75);
            font-size: 8px;
            letter-spacing: 4px;
        }

box {
    padding-left: 0;
    padding-right: 0;
}

label {
    color: white;
    font-size: 11px;
    font-weight: 500;
}
        "
    );

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
