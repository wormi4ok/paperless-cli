use std::env;
use std::path::PathBuf;
use std::process;

use clap::Parser;

const DEFAULT_PAPERLESS_URL: &str = "http://localhost:8000";
const API_UPLOAD_PATH: &str = "/api/documents/post_document/";

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
            process::exit(1)
        }
    };

    let paperless_url = env::var("PAPERLESS_URL").unwrap_or_else(|_| {
        println!("Warning: The PAPERLESS_URL environment variable is empty. Using default value.");
        DEFAULT_PAPERLESS_URL.to_string()
    });

    let config = PaperlessConfig {
        url: paperless_url,
        api_token: api_token.to_string(),
    };

    match upload_file(&config, args.filepath) {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn upload_file(paperless: &PaperlessConfig, file: PathBuf) -> Result<(), String> {
    let metadata = std::fs::metadata(&file).expect("file not found");

    let form = reqwest::blocking::multipart::Form::new()
        .file("document", &file)
        .unwrap();

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("{}{}", paperless.url, API_UPLOAD_PATH))
        .multipart(form)
        .header("Content-Length", metadata.len().to_string())
        .header("Authorization", format!("Token {}", paperless.api_token))
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if res.status().is_success() {
        println!("File uploaded successfully");
        Ok(())
    } else {
        let status = res.status();
        let message = res
            .text()
            .unwrap_or_else(|_| "Failed to read response text".to_string());
        Err(format!(
            "Error uploading file: {}. Server response: {}",
            status, message
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_upload_file_success() {
        let mut server = Server::new();
        let _mock = server
            .mock("POST", API_UPLOAD_PATH)
            .match_header("Authorization", "Token fake_token")
            .match_body(Matcher::Regex(".*".to_string()))
            .with_status(201)
            .create();

        let temp_file_path = create_temp_file("test_document.pdf", b"dummy content");
        let config = PaperlessConfig {
            url: server.url(),
            api_token: "fake_token".to_string(),
        };

        let result = upload_file(&config, temp_file_path.clone());
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_upload_file_failure() {
        let mut server = Server::new();
        let _mock = server
            .mock("POST", API_UPLOAD_PATH)
            .match_header("Authorization", "Token fake_token")
            .match_body(Matcher::Regex(".*".to_string()))
            .with_status(400)
            .with_body("Bad Request")
            .create();

        let temp_file_path = create_temp_file("test_document.pdf", b"dummy content");
        let config = PaperlessConfig {
            url: server.url(),
            api_token: "fake_token".to_string(),
        };

        let result = upload_file(&config, temp_file_path.clone());
        assert!(result.is_err());

        // Clean up
        std::fs::remove_file(temp_file_path).unwrap();
    }

    fn create_temp_file(file_name: &str, content: &[u8]) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(file_name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content).unwrap();
        file_path
    }
}
