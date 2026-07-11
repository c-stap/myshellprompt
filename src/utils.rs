use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum ShellType {
    Bash,
    Zsh,
    None,
}

pub fn wrap_ansi_for_bash(ansi_code: String) -> String {
    format!(r"\[{}\]", ansi_code)
}

pub fn wrap_ansi_for_zsh(ansi_code: String) -> String {
    format!("%{{{}%}}", ansi_code)
}

pub fn match_ansi_to_shell(shelltype: ShellType, ansi_code: String) -> String {
    match shelltype {
        ShellType::Bash => wrap_ansi_for_bash(ansi_code),
        ShellType::Zsh => wrap_ansi_for_zsh(ansi_code),
        ShellType::None => ansi_code,
    }
}

#[derive(Clone)]
pub struct Colour {
    pub fg: String,
    pub bg: String,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, shelltype: ShellType) -> Self {
        let mut fg_esc_code = format!(r"\e[38;2;{};{};{}m", r, g, b);
        let mut bg_esc_code = format!(r"\e[48;2;{};{};{}m", r, g, b);
        fg_esc_code = match_ansi_to_shell(shelltype.clone(), fg_esc_code);
        bg_esc_code = match_ansi_to_shell(shelltype.clone(), bg_esc_code);
        Self {
            fg: fg_esc_code,
            bg: bg_esc_code,
        }
    }
}
