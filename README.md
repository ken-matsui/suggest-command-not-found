# suggest-command-not-found

Typo correction for shell commands when command not found

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
   If you are using Bash, update your `~/.bash_profile`:
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
