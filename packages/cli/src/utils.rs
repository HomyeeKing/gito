use std::path::PathBuf;

use ini::Ini;
use std::fs;
// Removed: use crate::constants::DEFAULT_OPEN_CONFIG;

pub fn get_home_dir() -> PathBuf {
    match home::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory"),
    }
}

pub fn bug_report(command: &str) {
    println!(
        "{command} failed, please retry or report at https://github.com/HomyeeKing/gito/issues"
    )
}

pub fn get_config_path(path: &str) -> PathBuf {
    let gito_dir = get_home_dir().join(".gito");

    gito_dir.join(path)
}

pub fn safe_get_gito_config(path_str: &str) -> Ini {
    let config = match Ini::load_from_file(get_config_path(path_str)) {
        Ok(c) => c,
        Err(_) => Ini::new(),
    };
    return config;
}

pub fn init_gito_config() -> Result<(), String> {
    let gito_dir = get_home_dir().join(".gito");
    let account_file = gito_dir.join("account");
    let open_file = gito_dir.join("open");

    if !gito_dir.exists() {
        fs::create_dir_all(&gito_dir)
            .map_err(|e| format!("Failed to create .gito directory: {}", e))?;
    }

    if !account_file.exists() {
        fs::File::create(&account_file)
            .map_err(|e| format!("Failed to create account file: {}", e))?;
    }

    if !open_file.exists() {
        fs::File::create(&open_file)
            .map_err(|e| format!("Failed to create open file: {}", e))?;
    }

    Ok(())
}
