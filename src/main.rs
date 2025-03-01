use std::env;
use std::path::PathBuf;
use std::process;

use clap::Parser;

const DEFAULT_PAPERLESS_URL: &str = "http://localhost:8000";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filepath: PathBuf,
}

struct PaperlessConfig {
    url: String,
    api_token: String,
}

fn main() {
    let args = Cli::parse();
    if !&args.filepath.exists() {
        eprintln!("File {} not found", &args.filepath.display());
        process::exit(1);
    }

    let api_token = match env::var("PAPERLESS_API_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Couldn't find PAPERLESS_API_TOKEN: {}", e);
            process::exit(2)
        }
    };

    let paperless_url = env::var("PAPERLESS_URL").unwrap_or_else(|_| {
        println!("Warning: The PAPERLESS_URL environment variable is empty. Using default value.");
        DEFAULT_PAPERLESS_URL.to_string()
    });

    let config = PaperlessConfig {
        url: paperless_url,
        api_token: api_token.clone(),
    };

    let result = upload_file(config, args.filepath);
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(3);
    }
}

fn upload_file(paperless: PaperlessConfig, file: PathBuf) -> std::io::Result<()> {
    let metadata = std::fs::metadata(&file).expect("file not found");

    let form = reqwest::blocking::multipart::Form::new().file("document", &file)?;

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("{}/api/documents/post_document/", paperless.url))
        .multipart(form)
        .header("Content-Length", metadata.len().to_string())
        .header("Authorization", format!("Token {}", paperless.api_token))
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("File uploaded successfully");
            } else {
                eprintln!("Error uploading: {}", response.status());
                eprintln!(
                    "Message is: {}",
                    response
                        .text()
                        .unwrap_or_else(|_| "Failed to read response text".to_string())
                );
                process::exit(4);
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            process::exit(5);
        }
    }

    Ok(())
}
