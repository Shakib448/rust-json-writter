use clap::{Parser};


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


fn main() {
    println!("Hello, world!");
}
