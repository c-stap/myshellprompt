mod prompt;
mod themes;
mod os_icons;
mod sys_info;
pub mod utils;

use clap::Parser;

use crate::prompt::build_prompt;
use crate::themes::*;
use crate::utils::ShellType;

#[derive(Parser)]
#[command(name = "myshellprompt")]
#[command(about = "A simple prompt for bash and zsh with git and conda env integration", long_about = None)]
struct Args {
    #[arg(long, value_enum, help = "select shell type")]
    shell: Option<ShellType>,

    #[arg(long, value_enum)]
    theme: Option<ThemeType>,

    #[arg(long)]
    error: bool,
}

fn main() {
    let args = Args::parse();

    let shell = match args.shell {
        Some(ShellType::Bash) => ShellType::Bash,
        Some(ShellType::Zsh) => ShellType::Zsh,
        Some(ShellType::None) => ShellType::None,
        None => ShellType::None,
    };

    let theme = match args.theme {
        Some(ThemeType::SanzoWada329) => Theme::sanzo_wada_329(shell.clone()),
        Some(ThemeType::Greys) => Theme::greys(shell.clone()),
        Some(ThemeType::TokyonightMoon) => Theme::tokyonight_moon(shell.clone()),
        Some(ThemeType::TokyonightRainbow) => Theme::tokyonight_rainbow(shell.clone()),
        None => Theme::tokyonight_moon(shell.clone()),
    };

    let error = args.error;
    build_prompt(shell, theme, error);
}
