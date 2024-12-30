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
    url: String,
}

fn fetch_and_write_json(filename: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "output";
    let output_file_path = format!("{}/{}", output_dir, filename);

    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    let response = reqwest::blocking::get(url)?;
    let json_data: Value = response.json()?;

    let json_string = serde_json::to_string_pretty(&json_data)?;

    fs::write(&output_file_path, json_string)?;

    println!(
        "Fetched JSON data from {} and wrote it to: {}",
        url, output_file_path
    );


    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Failed to write JSON: {}", e);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    fetch_and_write_json(&cli.filename, &cli.url)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_fetch_and_write_json() {
        let test_url = "https://jsonplaceholder.typicode.com/todos/1";
        let test_filename = "tests_output.json";

        let result = fetch_and_write_json(test_filename, test_url);
        assert!(result.is_ok());

        let output_file_path = format!("output/{}", test_filename);
        assert!(Path::new(&output_file_path).exists());
        fs::remove_file(output_file_path).unwrap();
    }

    #[test]
    fn test_directory_creation() {
        let test_url = "https://jsonplaceholder.typicode.com/todos/1";
        let test_filename = "test_outputs.json";

        if Path::new("output").exists() {
            fs::remove_dir_all("output").unwrap();
        }

        let result = fetch_and_write_json(test_filename, test_url);
        assert!(result.is_ok());

        let output_file_path = format!("output/{}", test_filename);
        assert!(Path::new("output").exists());
        assert!(Path::new(&output_file_path).exists());

        fs::remove_file(output_file_path).unwrap();
        fs::remove_dir_all("output").unwrap();
    }

    #[test]
    fn test_invalid_url() {
        let test_url = "https://invalid.url";
        let test_filename = "test_output.json";
        let result = fetch_and_write_json(test_filename, test_url);
        assert!(result.is_err());
    }
}
