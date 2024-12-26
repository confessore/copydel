use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;

    // Traverse the directory recursively
    delete_items_with_copy(&current_dir)?;

    // Wait for user input before exiting
    println!("\nPress Enter to exit...");
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    Ok(())
}

fn delete_items_with_copy(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                if folder_name.contains("Copy") {
                    println!("Deleting directory: {}", path.display());
                    if let Err(e) = fs::remove_dir_all(&path) {
                        eprintln!("Failed to delete directory {}: {}", path.display(), e);
                    }
                    continue; // Skip further processing of this directory
                }
            }
            // Recur into subdirectories
            delete_items_with_copy(&path)?;
        } else if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if file_name.contains("Copy") {
                println!("Deleting file: {}", path.display());
                if let Err(e) = fs::remove_file(&path) {
                    eprintln!("Failed to delete file {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(())
}
