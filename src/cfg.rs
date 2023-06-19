use std::fs;
use std::io::ErrorKind;

pub const FILE_PATH: &str = "whitelisted_windows.txt";

pub type WindowTitlePart = String;

pub fn load_or_create_whitelisted_windows() -> Vec<WindowTitlePart> {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("Creating {FILE_PATH}.");
            let default_cfg = include_str!("../whitelisted_windows.txt");
            if let Err(err) = fs::write(FILE_PATH, default_cfg) {
                eprintln!("Could not create default whitelisted windows file: {err}");
            }
            default_cfg.to_string()
        },
        Err(err) => {
            eprintln!("Could not load whitelisted windows: {err}");
            String::new()
        }
    };
    file_content.lines().map(&str::to_string).collect()
}
