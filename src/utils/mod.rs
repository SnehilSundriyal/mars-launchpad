use std::process::Command;

pub fn launch_app(exec: &str) {
    let cleaned = exec
        .replace("%U", "")
        .replace("%u", "")
        .replace("%F", "")
        .replace("%f", "")
        .replace("%i", "")
        .replace("%c", "")
        .replace("%k", "")
        .trim()
        .to_string();

    if cleaned.is_empty() {
        return;
    }

    let args: Vec<String> = shell_words::split(&cleaned).unwrap_or_default();

    if args.is_empty() {
        return;
    }

    let program = &args[0];
    let rest = &args[1..];

    if let Err(err) = Command::new(program).args(rest).spawn() {
        eprintln!("Failed to launch {}: {}", program, err);
    }
}
