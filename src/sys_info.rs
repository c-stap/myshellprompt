use chrono::Local;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::os_icons;

pub fn get_active_python_env() -> String {
    // conda environment
    let mut conda_str = "".to_string();
    if let Ok(conda_env) = env::var("CONDA_DEFAULT_ENV") {
        conda_str = format!(" {}", conda_env);
    } else if let Ok(conda_prefix) = env::var("CONDA_PREFIX") {
        if let Some(name) = Path::new(&conda_prefix)
            .file_name()
            .and_then(|s| s.to_str())
        {
            conda_str = format!(" {}", name);
        }
    }

    // venv / virtualenv
    let mut venv_str = "".to_string();
    if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
        if let Some(name) = Path::new(&venv_path).file_name().and_then(|s| s.to_str()) {
            venv_str = format!("({}) ", name);
        }
    }

    format!("{}{}", venv_str, conda_str)
}

pub fn get_os() -> String {
    let os_name = os_icons::get_os_type();
    let os_icon = os_icons::get_os_icon(os_name);
    format!("{}", os_icon)
}

pub fn get_user_hostname() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
    format!("{}@{}", user, hostname)
}

pub fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}

pub fn get_pwd() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("unknown").to_path_buf());
    let home_dir = env::var("HOME").unwrap_or_else(|_| "unknown".to_string());

    let mut current_dir_str = current_dir.display().to_string();
    if current_dir_str.starts_with(&home_dir) {
        let rest = &current_dir_str[home_dir.len()..];
        current_dir_str = format!("~{}", rest);
    }
    format!("󰝰 {}", current_dir_str)
}

pub fn get_git_status() -> (String, bool) {
    let branch = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if branch.is_empty() {
        return ("".to_string(), true);
    }

    let status = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_default();

    let has_unstaged = status.lines().any(|line| {
        line.starts_with(" M") ||  // Modified, not staged
        line.starts_with(" D") ||  // Deleted, not staged
        line.starts_with("??") ||  // Untracked
        line.starts_with("!!") ||  // Ignored
        line.starts_with(" R") ||  // Renamed, not staged
        line.starts_with(" C") // Copied, not staged
    });

    let has_staged = status.lines().any(|line| {
        line.starts_with("M ") ||  // Modified, staged
        line.starts_with("D ") ||  // Deleted, staged
        line.starts_with("A ") ||  // Added, staged
        line.starts_with("R ") ||  // Renamed, staged
        line.starts_with("C ") // Copied, staged
    });

    let icon = if has_unstaged && has_staged {
        "󱇬󰦒"
    } else if has_unstaged {
        "󰦒"
    } else if has_staged {
        "󱇬"
    } else {
        ""
    };

    let branch_str = format!(" {} {}", branch, icon);
    let all_committed = status.is_empty();

    (branch_str, all_committed)
}
