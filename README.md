# suggest-command-not-found [![crates.io version](https://img.shields.io/crates/v/suggest-command-not-found.svg)](https://crates.io/crates/suggest-command-not-found) [![crates.io downloads](https://img.shields.io/crates/d/suggest-command-not-found.svg)](https://crates.io/crates/suggest-command-not-found)

Typo correction for shell commands when command not found

<img width="306" alt="ScreenShot" src="https://user-images.githubusercontent.com/26405363/234446161-5711b48c-7973-44b9-bb9e-f091baab1d8d.png">

## Usage

1. Install `suggest-command-not-found`
   ```console
   $ cargo install suggest-command-not-found
   ```
2. Add `command_not_found_handler` to your `~/.zshrc`
   ```zsh
   command_not_found_handler() {
     exec suggest-command-not-found $@
   }
   ```
   Or update your `~/.bash_profile` if you are using Bash:
   ```bash
   command_not_found_handle() {
     exec suggest-command-not-found $@
   }
   ```
3. Make a typo
   ```console
   $ carog new mypj
   Error: command not found: "carog" new mypj
      ==> Did you mean "cargo"?
   
   $ notacommand
   Error: command not found: "notacommand"
   
   $ echo $?
   127
   ```
