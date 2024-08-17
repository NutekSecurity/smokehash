# Smoke Hash

Check for file changes across your filesystem (Windows, Linux, macOS - if your system is not listed, try with cargo install or download source and build)

## Installation

```sh
cargo install smokehash
```

or use binary fro your OS from [Github releases page](https://github.com/NutekSecurity/smokehash/releases "Release Page")

## Use cases

* Automated trading bots logs - you can check for activity
* Check for automatically donloaded contente - if it changed
* Lookup your logs - maybe someone finnaly visited your website
* On penetration testing contracts - look for changes in extracted cookies, sql databases and such
* Overall changes in files - you just want to know

## Excerpt from help

```sh
smokehash 0.1.0
Neosb <neosb@nuteksecurity.com>
Check files for changes using SHA256 hash function using user provided lists file

PATH_TO_FILE1
PATH_TO_FILE2

Usage: smokehash [OPTIONS]

Options:
  -l, --list-file <LIST_FILE>          List of files you want to check for changes and at the same time storage file for SHA256 hash values of files
  -c, --create-list <CREATE_LIST>      Create list of files in directory
      --exclude-dirs <EXCLUDE_DIRS>    Exclude directories, especially useful when creating a list delimited by ',' - comma
      --exclude-files <EXCLUDE_FILES>  Exclude files, especially useful when creating a list delimited by ',' - comma
  -v, --verbose                        Should it echo files and status as it checks
  -h, --help                           Print help
  -V, --version                        Print version
```

## Roadmap

* Compartmentalize (make more functions) - right now program is in `src/main.rs` file
* Write proper unit tests, when comparmentalization is done
* Add `.gitignore` parsing as a bool flag to `clap` - which will look in every directory for a `.gitignore` file and will use it

## License

Apache-2.0 or MIT
