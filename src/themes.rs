use clap::ValueEnum;

use crate::utils::Color;

#[derive(Clone, ValueEnum)]
pub enum ThemeType {
    TokyonightMoon,
    Weird,
}

pub struct Theme {
    pub env_fg: Color,
    pub env_bg: Color,
    pub user_fg: Color,
    pub user_bg: Color,
    pub time_fg: Color,
    pub time_bg: Color,
    pub pwd_fg: Color,
    pub pwd_bg: Color,
    pub git_fg: Color,
    pub git_bg: Color,
    pub git_clean_fg: Color,
    pub git_clean_bg: Color,
}

impl Theme {
    pub fn tokyonight_moon(bash: bool) -> Self {
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

    pub fn weird(bash: bool) -> Self {
        let cream_yellow = Color::new(253, 191, 104, bash);
        let grayish_lavender_b = Color::new(192, 169, 179, bash);
        let cotinga_purple = Color::new(80, 19, 69, bash);
        let slate_color = Color::new(52, 69, 76, bash);
        let light_green_yellow = Color::new(199, 209, 79, bash);
        let white = Color::new(255, 255, 255, bash);
        let black = Color::new(0, 0, 0, bash);

        Theme {
            env_fg: black.clone(),
            env_bg: white.clone(),
            user_fg: black.clone(),
            user_bg: grayish_lavender_b,
            time_fg: white.clone(),
            time_bg: cotinga_purple,
            pwd_fg: white.clone(),
            pwd_bg: slate_color,
            git_fg: black.clone(),
            git_bg: cream_yellow,
            git_clean_fg: black.clone(),
            git_clean_bg: light_green_yellow,
        }
    }

}

