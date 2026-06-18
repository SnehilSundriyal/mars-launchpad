use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub desktop_file: PathBuf,
}

pub fn discover_apps() -> Vec<DesktopApp> {
    let mut apps = Vec::new();

    let home = std::env::var("HOME").unwrap_or_default();

    let dirs = [
        "/usr/share/applications".to_string(),
        format!("{}/.local/share/applications", home),
    ];

    for dir in dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }

            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Respect desktop entry metadata
            if content.contains("NoDisplay=true") {
                continue;
            }

            if content.contains("Hidden=true") {
                continue;
            }

            let mut name = String::new();
            let mut exec = String::new();
            let mut icon = String::new();

            for line in content.lines() {
                if let Some(v) = line.strip_prefix("Name=") {
                    name = v.to_string();
                }

                if let Some(v) = line.strip_prefix("Exec=") {
                    exec = v.to_string();
                }

                if let Some(v) = line.strip_prefix("Icon=") {
                    icon = v.to_string();
                }
            }

            // Must be launchable
            if exec.is_empty() || name.is_empty() {
                continue;
            }

            let lower = name.to_lowercase();

            // Hide Linux plumbing and helper entries
            let blacklist = [
                "oauth",
                "agent",
                "daemon",
                "applet",
                "launcher button",
                "app tray",
                "ibus",
                "kded",
                "knewstuff",
                "ktelnetservice",
                "handler for snap",
                "portal",
                "access prompt",
                "geoclue",
                "wayland",
            ];

            if blacklist.iter().any(|item| lower.contains(item)) {
                continue;
            }

            // Hide settings pages
            let settings_pages = [
                "accessibility",
                "appearance",
                "applications",
                "apps",
                "bluetooth",
                "color",
                "date & time",
                "date, time & calendar",
                "default applications",
                "desktop",
                "displays",
                "displays settings",
                "dock",
                "fonts",
                "input devices",
                "input method",
                "input sources",
                "keyboard",
                "keyboard layout",
                "language support",
                "mobile network",
                "mouse",
                "mouse & touchpad",
                "multitasking",
                "network",
                "network & wireless",
                "notifications",
                "online accounts",
                "panel",
                "power",
                "power & battery",
                "preferences",
                "privacy & security",
                "printers",
                "sound",
                "users",
            ];

            if settings_pages.contains(&lower.as_str()) {
                continue;
            }

            apps.push(DesktopApp {
                name,
                exec,
                icon,
                desktop_file: path.to_path_buf(),
            });
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));

    let mut seen = HashSet::new();
    apps.retain(|app| seen.insert(app.name.clone()));

    println!("Filtered to {} apps", apps.len());

    apps
}
