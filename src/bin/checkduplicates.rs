use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file_map = HashMap::new();
    let mut total_duplicates = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            // Remove the trailing " (number)" if it exists to get the base name
            let base_name = file_name.split(" (").next().unwrap_or(file_name);

            // Create a combined key of the base name and the extension
            let combined_key = format!("{}.{}", base_name, extension);
            let counter = file_map.entry(combined_key).or_insert(0);
            *counter += 1;
        }
    }

    for (file, count) in file_map {
        if count > 1 {
            total_duplicates += count - 1;
            println!("{} has {} duplicates", file, count - 1);
        }
    }

    println!("Directory {} has {} duplicates", path.display(), total_duplicates);

    Ok(())
}

