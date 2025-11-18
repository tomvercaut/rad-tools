# dcm_file_sort

A DICOM file sorting application that automatically organizes DICOM files from an input directory into a
structured output directory based on configurable path generation rules.

## Overview

`dcm_file_sort` is an CLI application that monitors an input directory for DICOM files and automatically moves them to
an output directory according to specified organizational rules. Files that cannot be processed as DICOM data are moved
to a separate `unknown` directory for manual review.

The application supports:

- Continuous monitoring of input directories
- Multiple DICOM path generation strategies (default and UZG formats)
- Configurable retry mechanisms for file operations
- Automatic handling of file conflicts with unique filename generation
- Graceful shutdown via Ctrl-C

## Usage

The application can be run in two modes: using a configuration file (recommended for consistency) or with manual command-line arguments.

### Using a Configuration File

Create a TOML configuration file (e.g., `config.toml`):

```toml
[paths]
# Input directory where DICOM files are initially stored and monitored
input_dir = "/path/to/input"

# Output directory where processed DICOM files will be organized
output_dir = "/path/to/output"

# Directory for files that cannot be processed as DICOM data
unknown_dir = "/path/to/unknown"

[path_generators]
# DICOM path generator type: "dicom_default" or "dicom_uzg"
dicom = "dicom_default"

[other]
# Time in milliseconds to wait after processing files before checking for new ones
wait_time_millisec = 500

# Time in milliseconds to wait before retrying file operations when the resource is busy
io_timeout_millisec = 500

# Number of attempts to copy a file if the resource is being used by another process
copy_attempts = 100

# Number of attempts to remove a file
remove_attempts = 10

# Seconds between last modified time and current time before a file is considered sortable
mtime_delay_secs = 10

# Maximum number of attempts to generate a unique filename in the output directory
limit_unique_filenames = 1000
```

Then run the service with the configuration file:

```bash
dcm_file_sort --config config.toml
```

## Using command line arguments

```bash
.\target\debug\dcm_file_sort --help
An application to sort DICOM data from an input directory into an output directory.

The way data is sorted depends on the path generator used.
Currently the following path generators are supported:
* dicom_default: Organizes DICOM files based on the patient ID and the date of birth.
* dicom_uzg: Organizes DICOM files based on the patient ID and the date of birth.

The name of the DICOM file is based on the modality and the (unique) SOP instance UID.

Logging can be enabled by setting the environment variable DCM_FILE_SORT_LOG to:
* TRACE
* DEBUG
* INFO
* WARN
* ERROR


Usage: dcm_file_sort [OPTIONS] --input-dir <INPUT_DIR> --output-dir <OUTPUT_DIR> --unknown-dir <UNKNOWN_DIR> --dicom-path-gen <DICOM_PATH_GEN>

Options:
  -i, --input-dir <INPUT_DIR>
          Directory where the DICOM data are read from

  -o, --output-dir <OUTPUT_DIR>
          Directory where the DICOM data is written to

  -u, --unknown-dir <UNKNOWN_DIR>
          Directory where files that could not be processed are moved

      --dicom-path-gen <DICOM_PATH_GEN>
          Path generator for DICOM data (accepted values: [DicomPathGeneratorType])

  -c, --config <CONFIG>
          

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Logging

Logging can be enabled by setting the `DCM_FILE_SORT_LOG` environment variable to one of the following log levels:

- `TRACE` - Most detailed logging, includes all trace-level messages
- `DEBUG` - Detailed debugging information
- `INFO` - General informational messages
- `WARN` - Warning messages for potentially problematic situations
- `ERROR` - Error messages for serious issues

### Setting the Environment Variable

**On Windows:**

```powershell
$env:DCM_FILE_SORT_LOG="TRACE"
```

**On Linux/macOS:**
```bash
DCM_FILE_SORT_LOG="TRACE"
```
