pub fn wrap_ansi_for_bash(ansi_code: String) -> String {
    format!(r"\[{}\]", ansi_code)
}

#[derive(Clone)]
pub struct Color {
    pub fg: String,
    pub bg: String,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, bash: bool) -> Self {
        let mut fg_esc_code = format!(r"\e[38;2;{};{};{}m", r, g, b);
        let mut bg_esc_code = format!(r"\e[48;2;{};{};{}m", r, g, b);
        if bash {
            fg_esc_code = wrap_ansi_for_bash(fg_esc_code);
            bg_esc_code = wrap_ansi_for_bash(bg_esc_code);
        }
        Self {
            fg: fg_esc_code,
            bg: bg_esc_code,
        }
    }
}
