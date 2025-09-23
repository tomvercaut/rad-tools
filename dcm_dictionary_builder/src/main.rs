use clap::Parser;
use rad_tools_dcm_dictionary_builder::{create_rs_file, create_tag_dictionary};
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::process::Command;

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
    match Command::new("rustfmt").arg(&output_path).status() {
        Ok(_) => println!("Formatted the tag dictionary file."),
        Err(e) => {
            if e.kind() != NotFound {
                println!("Failed to format the tag dictionary file: {}", e)
            }
        }
    }
    println!("The tag dictionary file has been created at {output_path:#?}.")
}
