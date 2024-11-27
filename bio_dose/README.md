# bio-dose

## Description
This crate provides a library and a binary to compute a biologically equivalent dose. At this moment this is limited to EQD2.

## Build

```shell
cargo build --release
```

## Usage

```shell
eqd2 --help

Computes equivalent dose in 2 Gy fractions:

EQD2 = D * ([d + a/b] / [2 + a/b])

where:
    - D: total dose in Gy.
    - d: dose per fraction in Gy
    - a/b: dose at which the linear and quadratic components of cell kill are equal in Gy


Usage: eqd2 [OPTIONS] --dose-per-fraction <DOSE> --n-fractions <TOTAL_NUMBER_FRACTIONS> --alpha-beta-ratio <ALPHA_BETA_RATIO>

Options:
  -d, --dose-per-fraction <DOSE>
          Dose per fraction in Gy

  -n, --n-fractions <TOTAL_NUMBER_FRACTIONS>
          Total number of fractions

  -a, --alpha-beta-ratio <ALPHA_BETA_RATIO>
          Dose (Gy) at which the linear and quadratic components of cell kill are equal

      --debug
          Enable logging at DEBUG level

      --trace
          Enable logging at TRACE level

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
