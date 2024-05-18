# One way sync

# Description

`one_way_sync` is a commandline application that synchronizes the data from one directory to another.

The application uses Robocopy and currently only works on Windows.


## Build

```shell
cargo build --release
```

## Usage

```shell
one-way-sync --help
A command line application that synchronizes the data from a source directory to a destination directory. The data is synchronized in one way / direction, it doesn't create a mirror between two directories.


Usage: one-way-sync [OPTIONS] --src <DIR> --dest <DIR>

Options:
  -s, --src <DIR>
          Source file or directory

  -d, --dest <DIR>
          Destination directory

  -l, --log
          Enable log

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
