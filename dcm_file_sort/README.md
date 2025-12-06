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

The application can be run using a configuration file.

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
# If the creation time is available on the filesystem, this will also be taken into account.
mtime_delay_secs = 10

# Maximum number of attempts to generate a unique filename in the output directory
limit_unique_filenames = 1000

# Limit the number of files being added for processing. If the limit is reached, the files are first moved into their new directories before searching for more files.
limit_max_processed_files = 1000
```

Then run the service with the configuration file:

```bash
dcm_file_sort --config config.toml
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
