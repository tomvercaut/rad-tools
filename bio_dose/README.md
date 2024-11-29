# bio-dose

## Description
This crate provides a library and a binaries to compute a biologically equivalent doses. At this moment this is limited to EQD2 and BED.

## Build

```shell
cargo build --release
```

## Usage

```shell
eqd2 --help
Computes equivalent dose in 2 Gy fractions in Gy:

EQD2 = D * ([d + a/b] / [2 + a/b])

where:
    - D: total dose in Gy.
    - d: dose per fraction in Gy
    - a/b: dose at which the linear and quadratic components of cell kill are equal in Gy


Usage: eqd2 [OPTIONS] -d <DOSE> -n <TOTAL_NUMBER_FRACTIONS> --ab <ALPHA_BETA_RATIO>

Options:
  -d <DOSE>
          Dose per fraction in Gy

  -n <TOTAL_NUMBER_FRACTIONS>
          Total number of fractions

  -a, --ab <ALPHA_BETA_RATIO>
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

```shell
bed --help
Computes biologically equivalent dose (BED) in Gy.

BED = D * (1 + d/[a/b])

where:
    - D: total dose in Gy.
    - d: dose per fraction in Gy
    - a/b: dose at which the linear and quadratic components of cell kill are equal in Gy


Usage: bed [OPTIONS] -d <DOSE> -n <TOTAL_NUMBER_FRACTIONS> -a <ALPHA_BETA_RATIO>

Options:
  -d <DOSE>
          Dose per fraction in Gy

  -n <TOTAL_NUMBER_FRACTIONS>
          Total number of fractions

  -a <ALPHA_BETA_RATIO>
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