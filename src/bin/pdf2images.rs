use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: pdf2images <pdf_file> [output_directory]");
        exit(1);
    }
    
    let pdf_path = &args[1];
    
    if !Path::new(pdf_path).exists() {
        eprintln!("Error: PDF file '{}' does not exist", pdf_path);
        exit(1);
    }
    
    let pdf_name = Path::new(pdf_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("pdf");
    
    let output_dir = if args.len() >= 3 {
        args[2].clone()
    } else {
        format!("{}-images", pdf_name)
    };
    
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("Error creating directory '{}': {}", output_dir, e);
        exit(1);
    }
    
    println!("Converting PDF '{}' to images in directory '{}'", pdf_path, output_dir);
    
    let output = Command::new("pdftoppm")
        .arg("-png")
        .arg(pdf_path)
        .arg(format!("{}/{}", output_dir, pdf_name))
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("Successfully converted PDF to images");
            } else {
                eprintln!("Error: pdftoppm failed");
                eprintln!("stderr: {}", String::from_utf8_lossy(&result.stderr));
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Could not execute pdftoppm. Make sure poppler-utils is installed.");
            eprintln!("Install with: brew install poppler");
            eprintln!("Error details: {}", e);
            exit(1);
        }
    }
}