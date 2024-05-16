# MPC checks cleaner

## Description

An application that cleans MPC directories created by Varian TrueBeam or Ethos systems on the VA_TRANSFER share. Old MPC Checks are removed if they are older than the number of specified days. The application also has a dry-run option that prints which files it would delete without doing so.

## Build

```shell
cargo build --release
```

## Usage

```shell
mpc_checks_cleaner.exe --help

A command line interface (CLI) application to clean the MPC checks in the VA_TRANSFER share.

The application removes old MPC checks from the VA_TRANSFER share. The application doesn't remove the Result.csv as it is small and can be usefull for external analysis. MPC checks are kept for a number of days before they are removed. Default value is 365 days.


Usage: mpc_checks_cleaner.exe [OPTIONS] --dir <DIR>

Options:
  -d, --dir <DIR>
          VA_TRANSFER share path

  -k, --keep <KEEP>
          Number of days the MPC checks are kept. Checks that are older, will be removed

          [default: 365]

      --dry-run
          Enable logging at DEBUG level

      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
