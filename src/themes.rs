use clap::ValueEnum;

use crate::utils::{Colour, ShellType};

#[derive(Clone, ValueEnum)]
pub enum ThemeType {
    TokyonightMoon,
    SanzoWada329,
}

pub struct Theme {
    pub env_fg: Colour,
    pub env_bg: Colour,
    pub user_fg: Colour,
    pub user_bg: Colour,
    pub time_fg: Colour,
    pub time_bg: Colour,
    pub pwd_fg: Colour,
    pub pwd_bg: Colour,
    pub git_fg: Colour,
    pub git_bg: Colour,
    pub git_clean_fg: Colour,
    pub git_clean_bg: Colour,
}

impl Theme {
    pub fn tokyonight_moon(shelltype: ShellType) -> Self {
        let white = Colour::new(255, 255, 255, shelltype.clone());
        let black = Colour::new(0, 0, 0, shelltype.clone());
        let pink = Colour::new(252, 167, 234, shelltype.clone());
        let magenta = Colour::new(192, 153, 255, shelltype.clone());
        let blue = Colour::new(130, 170, 255, shelltype.clone());
        // let orange = Colour::new(255, 150, 108, shelltype.clone());
        let yellow = Colour::new(255, 199, 119, shelltype.clone());
        let green = Colour::new(195, 232, 141, shelltype.clone());

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

    // Colour combination #329 from Sanzo Wada Dictionary of Colour Combinations
    // plus light green yellow from combination #311
    pub fn sanzo_wada_329(shelltype: ShellType) -> Self {
        let cream_yellow = Colour::new(253, 191, 104, shelltype.clone());
        let grayish_lavender_b = Colour::new(192, 169, 179, shelltype.clone());
        let cotinga_purple = Colour::new(80, 19, 69, shelltype.clone());
        let slate_color = Colour::new(52, 69, 76, shelltype.clone());
        let light_green_yellow = Colour::new(199, 209, 79, shelltype.clone());
        let white = Colour::new(255, 255, 255, shelltype.clone());
        let black = Colour::new(0, 0, 0, shelltype.clone());

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

