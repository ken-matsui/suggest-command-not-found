# suggest-command-not-found [![crates.io version](https://img.shields.io/crates/v/suggest-command-not-found.svg)](https://crates.io/crates/suggest-command-not-found) [![crates.io downloads](https://img.shields.io/crates/d/suggest-command-not-found.svg)](https://crates.io/crates/suggest-command-not-found)

Typo correction for not-found shell commands

<img width="306" alt="ScreenShot" src="https://user-images.githubusercontent.com/26405363/234446161-5711b48c-7973-44b9-bb9e-f091baab1d8d.png">

## Usage

1. Install `suggest-command-not-found`
   ```console
   $ cargo install suggest-command-not-found
   ```
2. Add `command_not_found_handler` to your `~/.zshrc`
   ```zsh
   command_not_found_handler() {
     if command -v suggest-command-not-found &> /dev/null; then
       exec suggest-command-not-found "$@"
     else
       echo "zsh: command not found: $*"
     fi
   }
   ```
   Or update your `~/.bash_profile` if you are using Bash:
   ```bash
   command_not_found_handle() {
     if command -v suggest-command-not-found &> /dev/null; then
       exec suggest-command-not-found "$@"
     else
       echo "bash: command not found: $*"
     fi
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

## Publish

### [GitHub Releases](https://github.com/ken-matsui/suggest-command-not-found/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

### [crates.io](https://crates.io/)

```bash
$ cargo publish
```
