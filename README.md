# git-codeowners

Parses GitHub CODEOWNERS[^1] files.

[^1]: See https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners

## Installation
```sh
cargo install --git https://github/triarius/git-codeowners.git
```

To invoke this as `git codeowners` instead of `/path/to/git-codeowners`, ensure that
1. `git` is installed and in your `PATH`
2. The location `cargo` installs executables to is in your `PATH`. This is typically `~/.local/share/cargo/bin`.

## Usage
Change directory to a git repo with a CODEOWNERS file.

```
Usage: git-codeowners [OPTIONS] <COMMAND>

Commands:
  find  Find owners for the specified paths. Reads paths from STDIN if not provided as positional arguments
  help  Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>  Path to the CODEOWNERS file [default: .github/CODEOWNERS]
  -h, --help         Print help
  -V, --version      Print version
```

### Detecting CODEOWNERS of all files changed in a branch
```sh
git diff (git merge-base origin/main HEAD) --name-only | git codeowners find
```
