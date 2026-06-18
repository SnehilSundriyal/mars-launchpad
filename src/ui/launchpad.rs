use crate::apps::discover_apps;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::SearchEntry;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FlowBox, Label, Orientation, ScrolledWindow};
const APPS_PER_PAGE: usize = 24;

pub fn build_ui(app: &Application) {
    load_css();

    let apps = discover_apps();

    let pages = Rc::new(
        apps.chunks(APPS_PER_PAGE)
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


    let flow = Rc::new(
        FlowBox::builder()
            .max_children_per_line(8)
            .selection_mode(gtk::SelectionMode::None)
            .column_spacing(24)
            .row_spacing(24)
            .build()
    );

    render_page(flow.as_ref(), &pages[0]);

    let scroll_controller =
        gtk::EventControllerScroll::new(
            gtk::EventControllerScrollFlags::VERTICAL,
        );

    let flow_scroll = flow.clone();
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

            render_page(
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

    let scroll = ScrolledWindow::builder()
        .child(flow.as_ref())
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

fn render_page(flow: &gtk::FlowBox, page: &[crate::apps::DesktopApp]) {
    while let Some(child) = flow.first_child() {
        flow.remove(&child);
    }

    for desktop_app in page {
        let exec = desktop_app.exec.clone();

        let tile = gtk::Box::new(gtk::Orientation::Vertical, 8);
        tile.set_halign(gtk::Align::Center);
        tile.set_valign(gtk::Align::Center);

        tile.set_size_request(140, 120);

        let icon_name = if desktop_app.icon.is_empty() {
            "application-x-executable"
        } else {
            &desktop_app.icon
        };

        let icon = gtk::Image::from_icon_name(icon_name);
        icon.set_pixel_size(96);

        let label = gtk::Label::new(Some(&desktop_app.name));

        label.set_wrap(true);
        label.set_max_width_chars(10);
        label.set_justify(gtk::Justification::Center);

        tile.append(&icon);
        tile.append(&label);

        let motion = gtk::EventControllerMotion::new();

        motion.connect_enter(|controller, _, _| {
            if let Some(widget) = controller.widget() {
                widget.set_cursor_from_name(Some("pointer"));
            }
        });

        motion.connect_leave(|controller| {
            if let Some(widget) = controller.widget() {
                widget.set_cursor_from_name(None);
            }
        });

        tile.add_controller(motion);

        let button = gtk::Button::new();

        button.set_cursor_from_name(Some("grab"));

        button.add_css_class("flat");
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
