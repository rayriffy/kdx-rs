use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = ".";
    process_directory(dir)?;
    Ok(())
}

fn process_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    println!("Getting files in {}", dir);
    
    // Get all files in the directory recursively
    let mut files: Vec<(String, u64)> = Vec::new();
    
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        
        // Skip if not a file
        if !path.is_file() {
            continue;
        }
        
        // Get relative path from the directory
        let relative_path = path.strip_prefix(dir)?;
        let path_str = relative_path.to_string_lossy();
        
        // Filter out specific files
        if path_str == "tree.csv" 
            || path_str == "miniserve-0.29.0.exe" 
            || path_str == "tree.py" {
            continue;
        }
        
        // Get file size
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        
        // Format path with leading "/" and convert backslashes to forward slashes
        let formatted_path = format!("/{}", path_str.replace('\\', "/"));
        
        files.push((formatted_path, size));
    }
    
    // Sort by filename
    files.sort_by(|a, b| a.0.cmp(&b.0));
    
    println!("Getting sizes for {} files", files.len());
    
    // Write to CSV
    let csv_path = Path::new(dir).join("tree.csv");
    println!("Writing to {}", csv_path.display());
    
    let mut wtr = Writer::from_path(csv_path)?;
    for (filename, size) in files {
        wtr.write_record(&[filename, size.to_string()])?;
    }
    wtr.flush()?;
    
    Ok(())
}
