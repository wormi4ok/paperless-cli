use std::env;
use std::path::PathBuf;

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
        println!("File {} not found", &args.filepath.display());
        std::process::exit(1);
    }

    let api_token = match env::var("PAPERLESS_API_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Couldn't find PAPERLESS_API_TOKEN: {}", e);
            return;
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
        println!("Error: {}", e);
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
        .send()
        .expect("request failed");

    if res.status().is_success() {
        println!("File uploaded successfully")
    } else {
        println!("Error uploading: {}", res.status());
        println!("Message is: {}", res.text().unwrap());
    }
    Ok(())
}
