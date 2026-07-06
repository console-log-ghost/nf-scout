use chrono::Local; // Local Time Interaction
use std::collections::HashMap; // Access HashMap components ins the standard library collections.
use std::env; // Standard Library - Environment Interaction
use std::fs; // File System
use std::path::Path; // More proper type for filesystem paths

#[derive(Debug)]
struct ScanResult {
    files: Vec<String>,
    stubs: Vec<String>,
}

fn main() {
    // Expandable text-value arguments variable, set to collect
    let args: Vec<String> = env::args().collect();

    // Confirms if folder path is provided, with a fallback process.
    if args.len() < 2 {
        println!("Please Provide a folder path.");
        return;
    }

    let project_path = &args[1]; // Uses provided folder path as project path. 
    
    let scan_result = scan_folder(Path::new(project_path)); 
    let found_files = &scan_result.files; // Vector count of found files, returns a Vec<String> of the paths.
    let stub_files = &scan_result.stubs;
    let categories = scan_categories(Path::new(project_path)); // Determined categories stored in a key-value relationship, allowing for a dynamic report of the folder structure.  
    let markdown_count = found_files.len(); // Count of found markdown files.

    // Report generation section, creates a String variable to hold the report text.
    let mut report = String::new();
    report.push_str("NF Scout Report\n\n");
    let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
    report.push_str(&format!("Generated: {}\n\n", timestamp));
    report.push_str(&format!("Project path:\n{}\n\n", project_path));
    report.push_str(&format!("Markdown files found: {}\n", markdown_count));
    report.push_str("\nCategories detected:\n");
    for (name, count) in &categories {
        report.push_str(&format!("- {}: {}\n", name, count));
    }
    if !stub_files.is_empty() {
        report.push_str("\nPossible issues:\n");
        report.push_str(&format!("Empty or stub files ({} found):\n", stub_files.len()));
        for stub in stub_files {
            report.push_str(&format!("- {}\n", stub));
        }
    }
        
    report.push_str("\nMarkdown Files:\n");
    for file in found_files {
        report.push_str(&format!("{}\n", file));
    }

    print!("{}", report);

    // Determine output path for the report file. If a second argument is provided, use it as the output path. 
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        let folder_name = Path::new(project_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("nf_scout"));
        format!("{}_report.md", folder_name)
    };
    fs::write(&output_path, &report).expect("Could not write report file");
    println!("Report saved to: {}", output_path)
}

fn is_stub_file(file_path: &Path) -> bool {
    if let Ok(contents) = fs::read_to_string(file_path) {
        contents.len() < 50
    } else {
        false
    }
}

// Scan a folder recursively for markdown files, returning a Vec<String> of the found file paths.
fn scan_folder(folder: &Path) -> ScanResult {
    // Read further into a scanned folder path and create an empty list to fill with found files. 
    let entries = fs::read_dir(folder).expect("Could not read folder");
    let mut found_files: Vec<String> = Vec::new(); // Vec::new() — creates an empty list to fill
    let mut stub_files: Vec<String> = Vec::new(); 

    // Item loop to read each entry in the folder, checking if it's a directory or a markdown file.
    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        if path.is_dir() {
            let mut sub_result = scan_folder(&path);
            found_files.append(&mut sub_result.files);
            stub_files.append(&mut sub_result.stubs);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            found_files.push(path.display().to_string());
            if is_stub_file(&path) {
                stub_files.push(path.display().to_string());
            }
        }
    }
    ScanResult {
        files: found_files,
        stubs: stub_files,
    }

}



// Scan the top-level folder for subfolders (categories) and count the number of markdown files in each.
fn scan_categories(folder: &Path) -> HashMap<String, usize> {
    
    // Create a HashMap to store category names and their corresponding markdown file counts.
    let mut categories: HashMap<String, usize> = HashMap::new();
    let entries = fs::read_dir(folder).expect("Could not read folder.");
    // Iterate through each entry in the folder.
    for entry in entries {
        let entry = entry.expect("Could not read entry.");
        let path = entry.path();

        // Check if the entry is a directory (category) and process it.
        if path.is_dir() {
            let folder_name_str = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if folder_name_str.starts_with('.') {
                continue;
            }
            let category_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("unknown"));
            let result = scan_folder(&path);
            categories.insert(category_name, result.files.len());
        }
    }
    categories // Send to main() for report generation
}

// ============================================================
// CONCEPTS FROM EARLIER — No Longer In Active Code
// ============================================================
//
// usize return type
//   scan_folder used to return just a count. Replaced by Vec<String>
//   so main gets the actual paths, not just a number.
//
// let mut markdown_count = 0  /  markdown_count += 1
//   Old manual counter declared with `mut`, incremented with `+= 1` per file found.
//   `+= 1` is a compound assignment — shorthand for `markdown_count = markdown_count + 1`.
//   Replaced by found_files.len(), which counts the Vec in one step.
//
// Hardcoded "Next step" footer
//   The first version printed a fixed string at the end of every report:
//   "Next step:\nReview the folder structure."
//   Removed when the report became dynamic. The footer served its purpose
//   as a training-wheels prompt but didn't scale.
//
// Printing inside scan_folder
//   Old code printed each file path directly as it found them.
//   scan_folder now collects and returns; main handles printing.
//
// One-layer scan (no recursion)
//   Original approach only read the top-level folder.
//   Replaced by the recursive scan_folder that digs into subfolders.
//
// println!("NF Scout starting...")
//   Abandoned welcome message from the very first version.
//
// ============================================================
