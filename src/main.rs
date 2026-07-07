mod themes;
mod prompt;
pub mod utils;

use clap::Parser;

use crate::themes::*;
use crate::prompt::build_prompt;

#[derive(Parser)]
#[command(name = "myshellprompt")]
#[command(about = "A simple prompt for bash and zsh with git and conda env integration", long_about = None)]
struct Args {
    #[arg(long, default_value_t = false, help = "formats prompt for BASH")]
    bash: bool,

    #[arg(long, value_enum)]
    theme: Option<ThemeType>,
}

fn main() {
    let args = Args::parse();
    let mut bash_mode = false;

    if args.bash {
        bash_mode = true;
    }

    let theme = match args.theme {
        Some(ThemeType::Weird) => Theme::weird(bash_mode),
        Some(ThemeType::TokyonightMoon) => Theme::tokyonight_moon(bash_mode),
        None => Theme::tokyonight_moon(bash_mode),
    };
    build_prompt(bash_mode, theme);
}
