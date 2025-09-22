use clap::Parser;
use rad_tools_dcm_dictionary_builder::{create_rs_file, create_tag_dictionary};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory path
    output_dir: PathBuf,
}

pub fn main() {
    let args = Args::parse();
    let dict = create_tag_dictionary().expect("Failed to create the tag dictionary.");
    let output_path = args.output_dir.join("tag.rs");
    create_rs_file(&output_path, &dict).expect("Failed to create the tag dictionary file.");
}
