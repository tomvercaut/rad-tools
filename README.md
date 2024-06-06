# rad-tools

An ecosystem of libraries and tools that can used in the field of radiotherapie.

## Components

### Libraries

The project at this stage has two libraries.

- [`radtools-ls-dcm`](ls_dcm) library used to read DICOM files by modality.
- [`radtools-bio-dose`](bio-dose) library to compute biological doses.

### Tools

The project at this stage has a couple command line tool.

- [`ls_rtplan`](ls_dcm) list RTPLAN DICOM files in a directory.
- [`eqd2`](eq2) compute the equivalent dose in 2 Gy fractions
- [`bed`](bed) compute the biological equivalent dose
- [`bed_time_factor`](bed_time_factor) compute the biological equivalent dose taking into account time and cell repopulation parameters.

## Building

You can use Cargo to build all the projects.

When you are building the project during the development stage, run `cargo build`. When you want to release the application, run `cargo build --release`

## Roadmap

Project is under active development.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
