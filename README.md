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
