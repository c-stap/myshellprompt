# myshellprompt

Simple shell prompt with git integration and support for python conda and venv environments for bash.

## Installation
Clone the repo, `cd` into the repo'a top directory and run
```bash
cargo install --path .
```

## Usage

In your `.bashrc` file, add the following line:
```bash
PROMPT_COMMAND='PS1="$(myshellprompt)"'
```

or if you want to add an icon for a -1 exit status for the previous command:
```bash
PROMPT_COMMAND='if [ $? -eq 0 ]; then PS1="$(myshellprompt)"; else PS1="\[\e[97m\]󰯆\[\e[0m\] $(myshellprompt)"; fi'
```
