use chrono::Local;
use std::env;
use std::path::Path;
use std::process::Command;

const RESET: &str = r"\e[0m";
const RESET_BG: &str = r"\e[49m";
// const RESET_FG: &str = "\e[39m";

const LEFT_SEMI_CIRCLE: &str = "";
const RIGHT_SEMI_CIRCLE: &str = "";

fn wrap_ansi_for_bash(ansi_code: String) -> String {
    format!(r"\[{}\]", ansi_code)
}

fn get_reset(bash: bool) -> String {
    if bash {
        wrap_ansi_for_bash(RESET.to_string())
    } else {
        RESET.to_string()
    }
}

fn get_reset_bg(bash: bool) -> String {
    if bash {
        wrap_ansi_for_bash(RESET_BG.to_string())
    } else {
        RESET_BG.to_string()
    }
}

#[derive(Clone)]
struct Color {
    // rgb: (u8, u8, u8),
    fg: String,
    bg: String,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, bash: bool) -> Self {
        let mut fg_esc_code = format!(r"\e[38;2;{};{};{}m", r, g, b);
        let mut bg_esc_code = format!(r"\e[48;2;{};{};{}m", r, g, b);
        if bash {
            fg_esc_code = wrap_ansi_for_bash(fg_esc_code);
            bg_esc_code = wrap_ansi_for_bash(bg_esc_code);
        }
        Self {
            // rgb: (r, g, b),
            fg: fg_esc_code,
            bg: bg_esc_code,
        }
    }
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

fn format_user_hostname_prompt(
    bg_color: &Color,
    fg_color: &Color,
    next_bg_color: &Color,
    env_prompt: &str,
) -> String {
    let user_hostname = get_user_hostname();
    let fmt_left_semi_circle = format!("{}{}", bg_color.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_color.bg, fg_color.fg, user_hostname);
    let fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);

    if env_prompt == "" {
        format!("{}{}{}", fmt_left_semi_circle, fmt_txt, fmt_sep)
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

fn format_pwd_prompt(
    bg_color: &Color,
    fg_color: &Color,
    next_bg_color: &Color,
    git_prompt: &str,
    bash: bool,
) -> String {
    let pwd_str = get_pwd();
    let fmt_txt = format!("{}{}{}", bg_color.bg, fg_color.fg, pwd_str);

    let reset_bg = get_reset_bg(bash);
    let fmt_sep: String;
    if git_prompt == "" {
        fmt_sep = format!("{}{}{}", bg_color.fg, reset_bg, RIGHT_SEMI_CIRCLE);
    } else {
        fmt_sep = format!("{}{}{}", bg_color.fg, next_bg_color.bg, RIGHT_SEMI_CIRCLE);
    }
    format!(" {}{}", fmt_txt, fmt_sep)
}

fn format_git_prompt(git_str: &str, fg_color: &Color, bg_color: &Color, bash: bool) -> String {
    let fmt_txt = format!("{}{}{}", fg_color.fg, bg_color.bg, git_str);

    let reset_bg = get_reset_bg(bash);
    let fmt_sep = format!("{}{}{}", bg_color.fg, reset_bg, RIGHT_SEMI_CIRCLE);
    if git_str != "" {
        format!(" {}{}", fmt_txt, fmt_sep)
    } else {
        "".to_string()
    }
}

struct Theme {
    env_fg: Color,
    env_bg: Color,
    user_fg: Color,
    user_bg: Color,
    time_fg: Color,
    time_bg: Color,
    pwd_fg: Color,
    pwd_bg: Color,
    git_fg: Color,
    git_bg: Color,
    git_clean_fg: Color,
    git_clean_bg: Color,
}

impl Theme {
    fn tokyonight_moon(bash: bool) -> Self {
        let white = Color::new(255, 255, 255, bash);
        let black = Color::new(0, 0, 0, bash);
        let pink = Color::new(252, 167, 234, bash);
        let magenta = Color::new(192, 153, 255, bash);
        let blue = Color::new(130, 170, 255, bash);
        // let orange = Color::new(255, 150, 108, bash);
        let yellow = Color::new(255, 199, 119, bash);
        let green = Color::new(195, 232, 141, bash);

        Theme {
            env_fg: black.clone(),
            env_bg: white,
            user_fg: black.clone(),
            user_bg: pink,
            time_fg: black.clone(),
            time_bg: magenta,
            pwd_fg: black.clone(),
            pwd_bg: blue,
            git_fg: black.clone(),
            git_bg: yellow,
            git_clean_fg: black.clone(),
            git_clean_bg: green,
        }
    }
}

fn build_prompt(bash: bool, theme: Theme) {
    let git_str: String;
    let all_committed: bool;
    (git_str, all_committed) = get_git_status();

    let env_prompt = format_env_prompt(&theme.env_bg, &theme.env_fg, &theme.user_bg);
    let user_host_prompt =
        format_user_hostname_prompt(&theme.user_bg, &theme.user_fg, &theme.time_bg, &env_prompt);
    let time_prompt = format_time_prompt(&theme.time_bg, &theme.time_fg, &theme.pwd_bg);

    let git_bg: Color;
    let git_fg: Color;
    if all_committed {
        git_bg = theme.git_clean_bg;
        git_fg = theme.git_clean_fg;
    } else {
        git_bg = theme.git_bg;
        git_fg = theme.git_fg
    }

    let git_prompt = format_git_prompt(&git_str, &git_fg, &git_bg, bash);
    let pwd_prompt = format_pwd_prompt(&theme.pwd_bg, &theme.pwd_fg, &git_bg, &git_prompt, bash);
    let reset = get_reset(bash);

    println!(
        "{}{}{}{}{}{} ",
        env_prompt, user_host_prompt, time_prompt, pwd_prompt, git_prompt, reset
    )
}

fn main() {
    let bash = true;
    let theme = Theme::tokyonight_moon(bash);
    build_prompt(bash, theme);
}
