use std::env;
use std::path::Path;
use std::process::Command;
use chrono::Local;

const RESET: &str = r"\[\e[0m\]";
const RESET_BG: &str = r"\[\e[49m\]";
// const RESET_FG: &str = "\e[39m";

const LEFT_SEMI_CIRCLE: &str = "";
const RIGHT_SEMI_CIRCLE: &str = "";

struct Color {
    // rgb: (u8, u8, u8),
    fg: String,
    bg: String,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            // rgb: (r, g, b),
            fg: format!(r"\[\e[38;2;{};{};{}m\]", r, g, b),
            bg: format!(r"\[\e[48;2;{};{};{}m\]", r, g, b),
        }
    }
}



fn get_active_python_env() -> String {
    // 1. Conda (highest priority if present)
    if let Ok(conda_env) = env::var("CONDA_DEFAULT_ENV") {
        return format!(" {}", conda_env);
    }

    // fallback conda detection
    if let Ok(conda_prefix) = env::var("CONDA_PREFIX") {
        if let Some(name) = Path::new(&conda_prefix)
            .file_name()
            .and_then(|s| s.to_str())
        {
            return format!(" {}", name);
        }
    }

    // 2. venv / virtualenv
    if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
        if let Some(name) = Path::new(&venv_path)
            .file_name()
            .and_then(|s| s.to_str())
        {
            return format!("({})", name);
        }
    }

    // 3. Poetry (optional common case)
    if env::var("POETRY_ACTIVE").is_ok() {
        return "poetry".to_string();
    }

    // 4. fallback
    "".to_string()
}


fn get_user_hostname() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
    format!("{}@{}", user, hostname)
}

fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}

fn get_pwd() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("unknown").to_path_buf());
    let home_dir = env::var("HOME").unwrap_or_else(|_| "unknown".to_string());

    let current_dir_str = current_dir.display().to_string();
    if current_dir_str.starts_with(&home_dir) {
        let rest = &current_dir_str[home_dir.len()..];
        format!("~{}", rest)
    } else {
        current_dir_str
    }
}

fn get_git_status() -> (String, bool) {
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

    // Check for unstaged changes (including untracked files)
    let has_unstaged = status.lines().any(|line| {
        line.starts_with(" M") ||  // Modified, not staged
        line.starts_with(" D") ||  // Deleted, not staged
        line.starts_with("??") ||  // Untracked
        line.starts_with("!!") ||  // Ignored
        line.starts_with(" R") ||  // Renamed, not staged
        line.starts_with(" C")     // Copied, not staged
    });

    // Check for staged changes
    let has_staged = status.lines().any(|line| {
        line.starts_with("M ") ||  // Modified, staged
        line.starts_with("D ") ||  // Deleted, staged
        line.starts_with("A ") ||  // Added, staged
        line.starts_with("R ") ||  // Renamed, staged
        line.starts_with("C ")     // Copied, staged
    });

    // Icon logic
    let icon = if has_unstaged && has_staged {
        "󱇬󰦒"  // Both staged and unstaged
    } else if has_unstaged {
        "󰦒"      // Only unstaged
    } else if has_staged {
        "󱇬"      // Only staged
    } else {
        ""         // Clean working directory
    };

    let branch_str = format!(" {} {}", branch, icon);
    let all_committed = status.is_empty();

    (branch_str, all_committed)
}

fn format_env_prompt(bg_color: &Color, fg_color: &Color, next_bg_color: &Color) -> String {
    let env_name = get_active_python_env();
    let fmt_left_semi_circle = format!("{}{}", bg_color.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_color.bg, fg_color.fg, env_name);
    let fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);

    if env_name == "" {
        "".to_string()
    } else {
        format!("{}{}{}", fmt_left_semi_circle, fmt_txt, fmt_sep)
    }
}

fn format_user_hostname_prompt(bg_color: &Color, fg_color: &Color, next_bg_color: &Color, env_prompt: &str) -> String {
    let user_hostname = get_user_hostname();
    let fmt_left_semi_circle = format!("{}{}", bg_color.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_color.bg, fg_color.fg, user_hostname);
    let fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);

    if env_prompt == "" {
        format!(" {}{}{}", fmt_left_semi_circle, fmt_txt, fmt_sep)
    } else {
        format!(" {}{}", fmt_txt, fmt_sep)
    }
}

fn format_time_prompt(bg_color: &Color, fg_color: &Color, next_bg_color: &Color) -> String {
    let time = get_time();
    let fmt_txt = format!("{}{} {}", bg_color.bg, fg_color.fg, time);
    let fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);

    format!(" {}{}", fmt_txt, fmt_sep)
}


fn format_pwd_prompt(bg_color: &Color, fg_color: &Color, next_bg_color: &Color, git_prompt: &str) -> String {
    let pwd_str = get_pwd();
    let fmt_txt = format!("{}{}{}", bg_color.bg, fg_color.fg, pwd_str);
    let fmt_sep: String;

    if git_prompt == "" {
        fmt_sep = format!("{}{}{}", bg_color.fg, RESET_BG, RIGHT_SEMI_CIRCLE);
    } else {
        fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);
    }
    format!(" {}{}", fmt_txt, fmt_sep)
}

fn format_git_prompt(git_str: &str, fg_color: &Color, bg_color: &Color) -> String {
    let fmt_txt = format!("{}{}{}", fg_color.fg, bg_color.bg, git_str);
    let fmt_sep = format!("{}{}{}", bg_color.fg, RESET_BG, RIGHT_SEMI_CIRCLE);
    if git_str != "" {
        format!(" {}{}", fmt_txt, fmt_sep)
    } else {
        "".to_string()
    }

}

fn main() {
    let white = Color::new(255, 255, 255);
    let black = Color::new(0, 0, 0);
    let pink = Color::new(252, 167, 234);
    let magenta = Color::new(192, 153, 255);
    let blue = Color::new(130, 170, 255);
    // let orange = Color::new(255, 150, 108);
    let yellow = Color::new(255, 199, 119);
    let green = Color::new(195, 232, 141);

    let git_str: String;
    let all_committed: bool;
    (git_str, all_committed) = get_git_status();

    let env_prompt = format_env_prompt(&white, &black, &pink);
    let user_host_prompt = format_user_hostname_prompt(&pink, &black, &magenta, &env_prompt);
    let time_prompt = format_time_prompt(&magenta, &black, &blue);

    let git_color: Color;
    if all_committed { git_color = green } else { git_color = yellow }
    let git_prompt = format_git_prompt(&git_str, &black, &git_color);
    let pwd_prompt = format_pwd_prompt(&blue, &black, &git_color, &git_prompt);

    println!("{}{}{}{}{}{} ", env_prompt, user_host_prompt, time_prompt, pwd_prompt, git_prompt, RESET)
}
