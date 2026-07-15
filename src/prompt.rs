use crate::themes::Theme;
use crate::utils::*;
use crate::sys_info::*;

const RESET: &str = r"\e[0m";
const RESET_BG: &str = r"\e[49m";

// const CLEAR: &str = r"\e[K";

const LEFT_SEMI_CIRCLE: &str = "";
const RIGHT_SEMI_CIRCLE: &str = "";
const RIGHT_TRIANGLE: &str = "";

fn get_reset(shelltype: ShellType) -> String {
    let reset = RESET.to_string();
    match_ansi_to_shell(shelltype, reset)
}

fn get_reset_bg(shelltype: ShellType) -> String {
    let reset = RESET_BG.to_string();
    match_ansi_to_shell(shelltype, reset)
}

// fn get_clear(shelltype: ShellType) -> String {
//     let clear = CLEAR.to_string();
//     match_ansi_to_shell(shelltype, clear)
// }

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

fn format_os_prompt(bg_colour: &Colour, fg_colour: &Colour, next_bg_colour: &Colour, env_prompt: &str) -> String {
    let os_icon = get_os();
    let fmt_left_semi_circle = format!("{}{}", bg_colour.fg, LEFT_SEMI_CIRCLE);
    let fmt_txt = format!("{}{}{}", bg_colour.bg, fg_colour.fg, os_icon);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_TRIANGLE);

    if env_prompt == "" {
        format!("{}{} {}", fmt_left_semi_circle, fmt_txt, fmt_sep)
    } else {
        format!(" {} {}", fmt_txt, fmt_sep)
    }

}

fn format_user_hostname_prompt(
    // sep_colour: &Colour,
    bg_colour: &Colour,
    fg_colour: &Colour,
    next_bg_colour: &Colour,
) -> String {
    let user_hostname = get_user_hostname();
    // let fmt_left_sep = format!("{}{}{}", bg_colour.bg, sep_colour.fg, RIGHT_TRIANGLE);
    let fmt_txt = format!(" {}{}{}", bg_colour.bg, fg_colour.fg, user_hostname);
    let fmt_sep = format!("{}{}{}", bg_colour.fg, next_bg_colour.bg, RIGHT_SEMI_CIRCLE);

    format!("{}{}", fmt_txt, fmt_sep)
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

    let env_prompt = format_env_prompt(&theme.env_bg, &theme.env_fg, &theme.os_bg);
    let os_prompt = format_os_prompt(&theme.os_bg, &theme.os_fg, &theme.user_bg, &env_prompt);
    let user_host_prompt =
        format_user_hostname_prompt(&theme.user_bg, &theme.user_fg, &theme.time_bg);
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
    // let clear = get_clear(shelltype.clone());
    let reset = get_reset(shelltype);

    print!(
        "╭─{}{}{}{}{}{}{}{}\n╰─>  ",
        error_sign, env_prompt, os_prompt, user_host_prompt, time_prompt, pwd_prompt, git_prompt, reset
    )
}
