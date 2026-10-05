use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for moving DoseCHECK PDFs.
#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct Args {
    #[arg(short, long, value_name = "INPUT_DIR", help = "Input directory")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR", help = "Output directory")]
    pub output: PathBuf,
    #[arg(
        short,
        long,
        value_name = "MILLISECONDS",
        help = "Watch for changes in the input directory (watch interval in milliseconds)."
    )]
    pub watch: Option<u64>,
}

impl Args {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.input.is_dir(), "Input directory does not exist");
        anyhow::ensure!(self.output.is_dir(), "Output directory does not exist");
        anyhow::ensure!(
            self.watch != Some(0),
            "Watch interval must be a positive integer"
        );
        Ok(())
    }
}
