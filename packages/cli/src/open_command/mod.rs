// TODO: seprate file
pub mod add {
    use crate::utils::{get_config_path, safe_get_gito_config};

    pub fn run(alias: &str, base_url: &str) {
        let mut config = safe_get_gito_config("open");
        config.set_to(Some(alias), "base_url".to_string(), base_url.to_string());
        config.write_to_file(get_config_path("open")).unwrap();
        println!("Add {alias} {base_url} successfully");
    }
}
pub mod del {
    use crate::utils::{get_config_path, safe_get_gito_config};
    pub fn run(alias: &str) {
        let mut config = safe_get_gito_config("open");
        config.delete(Some(alias));
        config.write_to_file(get_config_path("open")).unwrap();
        println!("Delete {alias} successfully");
    }
}

pub mod list {
    use gito_core::GitInfo;
    use prettytable::{color, row, Attr, Cell, Row, Table};
    use std::vec;
    use std::collections::HashSet; // Add this line

    use crate::constants::DEFAULT_OPEN_CONFIG; // Add this line

    use crate::utils::safe_get_gito_config;
    pub fn run(git_info: &GitInfo) {
        let mut git_account_table = Table::new();
        git_account_table.add_row(row!["alias", "base_url"]);

        let mut listed_aliases = HashSet::new();

        // List configurations from .gito/open
        let user_config = safe_get_gito_config("open");
        for (sec, prop) in user_config.iter() {
            let alias = sec.unwrap_or_default();
            let base_url = prop.get("base_url").unwrap_or_default();
            
            let mut group: Vec<Cell> = vec![];
            group.push(Cell::new(alias));
            group.push(Cell::new(base_url));

            if base_url == git_info.username { // Assuming git_info.username is used for highlighting
                for cell in group.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::GREEN));
                }
            }
            git_account_table.add_row(Row::new(group));
            listed_aliases.insert(alias.to_string());
        }

        // List default configurations, avoiding duplicates
        for (alias, url) in DEFAULT_OPEN_CONFIG.iter() {
            if !listed_aliases.contains(*alias) {
                let mut group: Vec<Cell> = vec![];
                group.push(Cell::new(alias));
                group.push(Cell::new(url));

                git_account_table.add_row(Row::new(group));
            }
        }
        
        git_account_table.printstd();
    }
}

pub mod open_website {
    use crate::utils::*;
    use gito_core::{utils::run_command, GitInfo}; // Changed run_git to run_command
    use crate::constants::DEFAULT_OPEN_CONFIG;

    pub fn run(alias: &str, git_info: &GitInfo) {
        let open_config = safe_get_gito_config("open");
        let mut base_url_found = None;

        // First, check user-defined configurations
        if let Some(base_url) = open_config.section(Some(alias)).and_then(|section| section.get("base_url")) {
            base_url_found = Some(base_url.to_string());
        }

        // If not found, check default configurations
        if base_url_found.is_none() {
            for (default_alias, default_url) in DEFAULT_OPEN_CONFIG.iter() {
                if default_alias == &alias {
                    base_url_found = Some(default_url.to_string());
                    break;
                }
            }
        }

        if let Some(mut base_url) = base_url_found {
            let url: String;
            if base_url.contains("<group>") || base_url.contains("<name>") {
                let parts: Vec<&str> = git_info.user_repo.split('/').collect();
                let group = parts.get(0).unwrap_or(&"");
                let name = parts.get(1).unwrap_or(&"");

                url = base_url
                    .replace("<group>", group)
                    .replace("<name>", name);
            } else {
                // Ensure base_url ends with a slash if no placeholders are used
                if !base_url.ends_with('/') {
                    base_url.push('/');
                }
                url = format!("{}{}", base_url, git_info.user_repo);
            }
            
            #[cfg(target_os = "macos")]
            let output = run_command("open", vec![&url]);
            #[cfg(target_os = "linux")]
            let output = run_command("xdg-open", vec![&url]);
            #[cfg(target_os = "windows")]
            let output = run_command("start", vec!["", &url]); // "start" often needs an empty first arg for title

            if !output.status.success() {
                eprintln!("Failed to open URL: {}", String::from_utf8_lossy(&output.stderr));
            } else {
                println!("Opening: {}", url);
            }
        } else {
            eprintln!("No base URL found for alias: {}", alias);
        }
    }
}
