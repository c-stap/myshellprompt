use chrono::Local;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::themes::Theme;
use crate::utils::*;

const RESET: &str = r"\e[0m";
const RESET_BG: &str = r"\e[49m";
const CLEAR: &str = r"\e[K";

const LEFT_SEMI_CIRCLE: &str = "";
const RIGHT_SEMI_CIRCLE: &str = "";

fn get_reset(shelltype: ShellType) -> String {
    let reset = RESET.to_string();
    match_ansi_to_shell(shelltype, reset)
}

fn get_reset_bg(shelltype: ShellType) -> String {
    let reset = RESET_BG.to_string();
    match_ansi_to_shell(shelltype, reset)
}

fn get_clear(shelltype: ShellType) -> String {
    let clear = CLEAR.to_string();
    match_ansi_to_shell(shelltype, clear)
}

fn get_active_python_env() -> String {
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

fn get_user_hostname() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
    format!(" {}@{}", user, hostname)
}

fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}

fn get_pwd() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("unknown").to_path_buf());
    let home_dir = env::var("HOME").unwrap_or_else(|_| "unknown".to_string());

    let mut current_dir_str = current_dir.display().to_string();
    if current_dir_str.starts_with(&home_dir) {
        let rest = &current_dir_str[home_dir.len()..];
        current_dir_str = format!("~{}", rest);
    }
    format!("󰝰 {}", current_dir_str)
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

fn format_env_prompt(bg_colour: &Colour, fg_colour: &Colour, next_bg_colour: &Colour) -> String {
    let env_name = get_active_python_env();
    let fmt_left_semi_circle = format!("{}{}", bg_colour.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_colour.bg, fg_colour.fg, env_name);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_SEMI_CIRCLE);

    if env_name == "" {
        "".to_string()
    } else {
        format!("{}{}{}", fmt_left_semi_circle, fmt_txt, fmt_sep)
    }
}

fn format_user_hostname_prompt(
    bg_colour: &Colour,
    fg_colour: &Colour,
    next_bg_colour: &Colour,
    env_prompt: &str,
) -> String {
    let user_hostname = get_user_hostname();
    let fmt_left_semi_circle = format!("{}{}", bg_colour.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_colour.bg, fg_colour.fg, user_hostname);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_SEMI_CIRCLE);

    if env_prompt == "" {
        format!("{}{}{}", fmt_left_semi_circle, fmt_txt, fmt_sep)
    } else {
        format!(" {}{}", fmt_txt, fmt_sep)
    }
}

fn format_time_prompt(bg_colour: &Colour, fg_colour: &Colour, next_bg_colour: &Colour) -> String {
    let time = get_time();
    let fmt_txt = format!("{}{} {}", bg_colour.bg, fg_colour.fg, time);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_SEMI_CIRCLE);

    format!(" {}{}", fmt_txt, fmt_sep)
}

fn format_pwd_prompt(
    bg_colour: &Colour,
    fg_colour: &Colour,
    next_bg_colour: &Colour,
    git_prompt: &str,
    shelltype: ShellType,
) -> String {
    let pwd_str = get_pwd();
    let fmt_txt = format!("{}{}{}", bg_colour.bg, fg_colour.fg, pwd_str);

    let reset_bg = get_reset_bg(shelltype);
    let fmt_sep: String;
    if git_prompt == "" {
        fmt_sep = format!("{}{}{}", bg_colour.fg, reset_bg, RIGHT_SEMI_CIRCLE);
    } else {
        fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_SEMI_CIRCLE);
    }
    format!(" {}{}", fmt_txt, fmt_sep)
}

fn format_git_prompt(
    git_str: &str,
    fg_colour: &Colour,
    bg_colour: &Colour,
    shelltype: ShellType,
) -> String {
    let fmt_txt = format!("{}{}{}", fg_colour.fg, bg_colour.bg, git_str);

    let reset_bg = get_reset_bg(shelltype);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, reset_bg, RIGHT_SEMI_CIRCLE);
    if git_str != "" {
        format!(" {}{}", fmt_txt, fmt_sep)
    } else {
        "".to_string()
    }
}

pub fn build_prompt(shelltype: ShellType, theme: Theme, error: bool) {
    let mut error_sign = String::from("");
    if error {
        error_sign.push_str(" 󰯆 ")
    }
    let git_str: String;
    let all_committed: bool;
    (git_str, all_committed) = get_git_status();

    let env_prompt = format_env_prompt(&theme.env_bg, &theme.env_fg, &theme.user_bg);
    let user_host_prompt =
        format_user_hostname_prompt(&theme.user_bg, &theme.user_fg, &theme.time_bg, &env_prompt);
    let time_prompt = format_time_prompt(&theme.time_bg, &theme.time_fg, &theme.pwd_bg);

    let git_bg: Colour;
    let git_fg: Colour;
    if all_committed {
        git_bg = theme.git_clean_bg;
        git_fg = theme.git_clean_fg;
    } else {
        git_bg = theme.git_bg;
        git_fg = theme.git_fg
    }

    let git_prompt = format_git_prompt(&git_str, &git_fg, &git_bg, shelltype.clone());
    let pwd_prompt = format_pwd_prompt(
        &theme.pwd_bg,
        &theme.pwd_fg,
        &git_bg,
        &git_prompt,
        shelltype.clone(),
    );
    let clear = get_clear(shelltype.clone());
    let reset = get_reset(shelltype);

    println!(
        "╭─{}{}{}{}{}{}{}{}\n╰─>  ",
        error_sign, env_prompt, user_host_prompt, time_prompt, pwd_prompt, git_prompt, clear, reset
    )
}
