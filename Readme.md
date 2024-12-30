# Fetch and Write JSON

This project fetches arbitrary JSON data from a specified URL and writes it to a file in the output folder. It is built using Rust and leverages libraries such as `clap` for command-line argument parsing, `reqwest` for HTTP requests, and `serde_json` for JSON handling.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/learn/get-started) if you haven't already.
- **Cargo**: Cargo is the Rust package manager and build system, and it comes installed with Rust.

## Installation

Follow these steps to set up and run the project:

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/Shakib448/fetch-and-write-json.git
    cd fetch-and-write-json
    ```

2. **Build the Project**:
    ```sh
    cargo build --release
    ```

## Usage

To use this project, follow these steps:

1. **Run the Application**:
    ```sh
    cargo run --release -- --filename <output_filename> --url <json_url>
    ```

   Replace `<output_filename>` with the name you want for the output JSON file and `<json_url>` with the URL from which you want to fetch the JSON data.

### Example

To fetch JSON data from `https://jsonplaceholder.typicode.com/todos/1` and write it to a file named `output.json`, run:

```sh
cargo run --release -- --filename output.json --url https://jsonplaceholder.typicode.com/todos/1