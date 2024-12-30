use clap::{Parser};
use std::{fs, path::Path};
use serde_json::Value;

#[derive(Parser)]
#[command(
    name = "Fetch and Write JSON",
    version = "1.0",
    about = "Fetches arbitrary JSON data and writes it to a file in the output folder."
)]
struct Cli {
    /// Name of the output json file
    #[arg(short, long)]
    filename: String,

    /// URL to fetch JSON data from
    #[arg(short, long)]
    url : String
}


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let output_dir = "output";
    let output_file_path = format!("{}/{}", output_dir, cli.filename);

    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    let response = reqwest::blocking::get(&cli.url)?;
    let json_data: Value = response.json()?;

    let json_string = serde_json::to_string_pretty(&json_data)?;

    fs::write(&output_file_path, json_string)?;

    println!(
        "Fetched JSON data from {} and wrote it to: {}",
        cli.url, output_file_path
    );


    Ok(())
}
