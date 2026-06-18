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

    let dirs = [
        "/usr/share/applications",
        "/home/snehil/.local/share/applications",
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

            if !name.is_empty() {
                apps.push(DesktopApp {
                    name,
                    exec,
                    icon,
                    desktop_file: path.to_path_buf(),
                });
            }
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    let mut seen = HashSet::new();

    apps.retain(|app| seen.insert(app.name.clone()));
    apps
}
