# pycleaner
Simple tool to delete all __pycache__ directories.

Important note: it won't follow symlinks

## How to install
Simply run
```bash
cargo install --git https://github.com/nullyfae/pycleaner
```

## Usage

```bash
pycleanner

USAGE:
    pycleanner.exe [OPTIONS]

OPTIONS:
        --dry                   Makes it dry run
    -h, --help                  Print help information
    -l, --loc <LOCALIZATION>    Runs the program starting from other path than the current dir
    -n <MAX_DEPTH>
```
