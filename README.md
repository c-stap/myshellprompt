# myshellprompt

Simple shell prompt with git integration and support for python conda and venv environments for bash and zsh.

## Installation
Make sure to have rust and cargo installed.

Clone the repo, `cd` into the repo's top directory and run
```bash
cargo install --path .
```

## Usage

### For BASH
In your `.bashrc` file, add the following line:
```bash
PROMPT_COMMAND='PS1="$(myshellprompt --shell bash)"'
```

or if you want to add an icon for a -1 exit status for the previous command:
```bash
alias shellprompt='myshellprompt --bash'
PROMPT_COMMAND='if [ $? -eq 0 ]; then PS1="$(shellprompt)"; else PS1="\[\e[97m\]󰯆\[\e[0m\] $(shellprompt)"; fi'
```

To change the theme:
```bash
alias shellprompt='myshellprompt --bash --theme tokyonight-moon'
PROMPT_COMMAND='if [ $? -eq 0 ]; then PS1="$(shellprompt)"; else PS1="\[\e[97m\]󰯆\[\e[0m\] $(shellprompt)"; fi
```

### For ZSH
In your `.zshrc` file, add the following line:
```zsh
precmd() { PROMPT=$(echo -e "$(myshellprompt)") }
```

or if you want to add an icon for a -1 exit status for the previous command:
```zsh
alias shellprompt='myshellprompt --shell zsh'
precmd() {
    if [ $? -eq 0 ]; then
        PROMPT=$(echo -e "$(shellprompt)")
    else
        PROMPT="%B󰯆%b $(echo -e "$(shellprompt)")"
    fi
}
```

To change the theme:
```zsh
alias shellprompt='myshellprompt --shell zsh --theme tokyonight-moon'
precmd() {
    if [ $? -eq 0 ]; then
        PROMPT=$(echo -e "$(shellprompt)")
    else
        PROMPT="%B󰯆%b $(echo -e "$(shellprompt)")"
    fi
}
```
