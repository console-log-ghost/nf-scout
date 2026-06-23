use chrono::Local; // Local Time Interaction
use std::collections::HashMap; // Access HashMap components ins the standard library collections.
use std::env; // Standard Library - Environment Interaction
use std::fs; // File System
use std::path::Path; // More proper type for filesystem paths

fn main() {
    // Creates an arguments variable, adds growable text values to the variable,
    // and tells it that it is interacting with the environment to "collect".
    let args: Vec<String> = env::args().collect();

    // Identifies if it received a folder path, if not indicates that it requires
    // a folder path, and then returns.
    if args.len() < 2 {
        println!("Please Provide a folder path.");
        return;
    }

    // Borrows the first user-provided command-line argument as the project path.
    let project_path = &args[1];

    let found_files = scan_folder(Path::new(project_path));
    let categories = scan_categories(Path::new(project_path));
    let markdown_count = found_files.len();

    // New `mut` variable ─`report`─ creates a string, stores strings ─push_str()─, gives back strings ─&format!─,
    // Uses `\n`to create new-line characters inside a string, replaces `println!()` used to create a creak in the text.
    // Uses chrono to stamp time of report generation, prints it on report it.
    // Uses `print!` instead of `println!` since we put breaks already baked in the strings ─ `\n`

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
    report.push_str("\nMarkdown Files:\n");
    for file in &found_files {
        report.push_str(&format!("{}\n", file));
    }

    print!("{}", report);

    // variable ─`output_path`─, does a safe check `args.en() >2`, 
    //if user provides a file path it will print to that folder under a provided files nale.
    // If user does not provide a path, it will fall back to creating the report in the path folder
    // Takes the project folder name and uses it to name the report `{project folder name}_report.md`
    //expect associated error.
    // prints to the path
    
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

// Vec<String> — a growable list of text values holding the found file paths.
// Alternative to the old `usize` return, which only gave back a count.
fn scan_folder(folder: &Path) -> Vec<String> {
    // Read further into a scanned folder path
    let entries = fs::read_dir(folder).expect("Could not read folder"); 
    let mut found_files: Vec<String> = Vec::new(); // Vec::new() — creates an empty list to fill

    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();

        // My understanding: If a path is a dir/folder, identify as sub_files — a Vec<String>.
        // Recursively scan through all connected paths. When there is no more to scan,
        // if the extension is `md`, push the path as text into the list.
        if path.is_dir() {
            let mut sub_files = scan_folder(&path);
            found_files.append(&mut sub_files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            found_files.push(path.display().to_string());
        }
    }
    found_files // No `;` — returns the list to the function caller.
}

// Creates a function that takes a borrowed folder path and returns a HashMap —
// a key-value store where the key is the category name (folder name as a String)
// and the value is how many .md files live inside it (a usize count).
// The `mut` on categories means we're going to fill it up as we go — it starts empty.
// `entries` is the raw list of everything directly inside the top-level folder.
fn scan_categories(folder: &Path) -> HashMap<String, usize> {
    let mut categories: HashMap<String, usize> = HashMap::new();
    let entries = fs::read_dir(folder).expect("Could not read folder.");

    // Loop over every item directly inside the folder (not recursively — just the top layer).
    // Each `entry` is unwrapped from a Result because reading from disk can fail.
    // `path` is that entry's full filesystem path.
    for entry in entries {
        let entry = entry.expect("Could not read entry.");
        let path = entry.path();

        // Only care about directories — skip loose files at the root level.
        // If it is a directory: extract just the folder's own name (not the full path),
        // convert it to a String we can own and store.
        // .map() transforms the name if it exists. .unwrap_or_else() is the fallback —
        // if file_name() returns nothing (rare edge case), use "unknown" instead of crashing.
        if path.is_dir() {
            let category_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("unknown"));

            // Reuse scan_folder to recursively count all .md files inside this subfolder.
            // `files` is a Vec<String> of those paths. We only need the count, so .len() gives us that.
            // .insert() stores the pair: folder name → file count into the HashMap.
            let files = scan_folder(&path);
            categories.insert(category_name, files.len());
        }
    }
    // No semicolon — this returns `categories` to whoever called the function.
    categories
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
