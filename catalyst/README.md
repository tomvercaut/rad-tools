# Catalyst couch calibration smoother

## Description

A tool to read the couch calibration from a Catalyst system.
The amplitude and couch position is smoothed using a central moving average filter.
After processing the data is written in CSV format to a file.

## Build

```shell
cargo build --release
```

## Usage

```shell
catalyst_smooth_couch_calibration.exe --help
Smooth the catalyst couch calibration data.

Smoothing the catalyst couch calibration data is done by computing a central moving average of the input.


Usage: catalyst_smooth_couch_calibration.exe [OPTIONS] --input <INPUT> --output <OUTPUT> --cma-nw <CMA_NW> --cma-w <CMA_W>

Options:
  -i, --input <INPUT>
          Input couch calibration file

  -o, --output <OUTPUT>
          Output CSV couch calibration file

      --cma-nw <CMA_NW>
          Central moving average width for the no weight measurements

          Width can be specified in absolute values (e.g. 3) or as a percentage (e.g.: 2% or 2p) of all the measurement points.

      --cma-w <CMA_W>
          Central moving average width for the weight data measurements

          Width can be specified in absolute values (e.g. 3) or as a percentage (e.g.: 2% or 2p) of all the measurement points.

  -v, --verbose
          Enable verbose output

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
