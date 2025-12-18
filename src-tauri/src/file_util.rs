use glob::glob;
use regex::Regex;
use std::fmt;
use std::io;
use std::fs;
use std::fs::metadata;
use std::path::Path;

/// Custom error type (simplified error handling)
#[derive(Debug)]
enum CheckFileError {
    InvalidPath,          // Invalid folder path
    DirectoryNotFound,    // Folder does not exist
    GlobError(glob::PatternError), // Wildcard pattern parsing error
    GlobIterError(glob::GlobError),       // Glob iteration error
}

// Implement error formatting (for easy printing)
impl fmt::Display for CheckFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckFileError::InvalidPath => write!(f, "Invalid folder path"),
            CheckFileError::DirectoryNotFound => write!(f, "Directory not found"),
            CheckFileError::GlobError(e) => write!(f, "Wildcard pattern error: {}", e),
            CheckFileError::GlobIterError(e) => write!(f, "Glob iteration error: {}", e),
        }
    }
}

pub fn check_file_exist(file_path: &str) -> bool {
    let path = Path::new(file_path);

    let file_metadata = match metadata(path) {
        Ok(meta) => meta,
        Err(_) => return false,
    };

    file_metadata.is_file() && file_metadata.len() > 0
}

pub fn check_folder_exist(path_str: &str) -> bool {
    let path = Path::new(path_str);
    
    match path.try_exists() {
        Ok(exists) => exists,
        Err(_) => false,
    }
}

pub fn create_dir_if_not_exists(path: &str) -> io::Result<()> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    Ok(())
}

pub fn write_to_file(path: &str, output_dir: &str, content: &str) {
    if let Err(e) = create_dir_if_not_exists(output_dir) {
        println!("create {} dir failed:{}", output_dir, e);
    }
    let file_name = format!("{}/{}", &output_dir, &path);
    println!("file:{}", &file_name);
    if let Err(e) = fs::write(&file_name, content) {
        eprintln!("write file{}failed:{}", file_name, e);
    } else {
        println!("write file success:{}", file_name);
    }
}


pub fn read_text_file(file_path: &str) -> Result<String, String> {
    // Convert string path to Path object for validation
    let path = Path::new(file_path);

    // Validate path exists before reading
    if !path.exists() {
        return Err(format!("File does not exist: '{}'", file_path));
    }

    // Validate path is a file (not a directory)
    if !path.is_file() {
        return Err(format!("Path is a directory, not a file: '{}'", file_path));
    }

    // Read file content to string with error handling
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            // Map system errors to human-readable messages
            let error_msg = match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: Cannot read file '{}'", file_path)
                }
                std::io::ErrorKind::NotFound => {
                    format!("File not found: '{}'", file_path)
                }
                std::io::ErrorKind::InvalidData => {
                    format!("Invalid text encoding: File '{}' is not a valid text file", file_path)
                }
                _ => format!(
                    "Failed to read file '{}': {}",
                    file_path, e
                ),
            };
            Err(error_msg)
        }
    }
}

pub fn parse_file_path(full_path: &str) -> (String, String) {
    // Convert the input string to a Path object for path manipulation
    let path = Path::new(full_path);
    if check_file_exist(full_path) == false {
        return ("".to_string(), "".to_string());
    }

    // 1. Extract the file name from the path with error handling
    let file_name = path
        .file_name()
        .ok_or_else(|| format!("Failed to extract file name from path '{}'", full_path)) // Return error string
        .and_then(|os_str| {
            // Convert OsStr to &str, return error if invalid Unicode
            os_str.to_str()
                .ok_or_else(|| format!("Path '{}' contains invalid Unicode characters", full_path))
        })
        .unwrap() // Panic on error (simple error handling for this example)
        .to_string();

    // 2. Extract the parent directory of the file with error handling
    let directory = path
        .parent()
        .ok_or_else(|| format!("Failed to extract directory from path '{}' (may be root directory)", full_path))
        .and_then(|path| {
            // Convert Path to &str, return error if invalid Unicode
            path.to_str()
                .ok_or_else(|| format!("Directory path '{}' contains invalid Unicode characters", full_path))
        })
        .unwrap() // Panic on error (simple error handling for this example)
        .to_string();

    (file_name, directory)
}

pub fn analysis_info(input: &str) -> String {
    let mut output = String::new();
    let re = Regex::new(r"0x[0-9a-fA-F]+").expect("Reg compile failed");
    input.lines().for_each(|line| {
        if line.contains("Device Total Physical Partitions") {
            if let Some(hex_match) = re.find(line) {
                let hex_str = hex_match.as_str();
                match u64::from_str_radix(&hex_str[2..], 16) {
                    Ok(decimal) => { output = format!("{}\n Device Total Physical Partitions:{}", output, decimal);},
                    Err(e) => eprintln!("Convert Failed: {}", e),
                }
            }
            println!("{}", line);
        } else if line.contains("Device Serial Number") {
            if let Some(hex_match) = re.find(line) {
                let hex_str = hex_match.as_str();
                output = format!("{}\n Device Serial Number:{}", output, hex_str);
            }
        } else if line.contains("UFS Inquiry Command Output") {
            if let Some((_, content)) = line.split_once("Output:") {
                let model = content.replace("'", " ");
                output = format!("{}\n Storage:{}", output, model.trim());
            }
        } else if line.contains("Boot Partition Enabled") {
            if let Some(hex_match) = re.find(line) {
                let hex_str = hex_match.as_str();
                match u64::from_str_radix(&hex_str[2..], 16) {
                    Ok(decimal) => {
                        let slot = if decimal == 1 {
                            "A"
                        } else {
                            "B"
                        };
                        output = format!("{}\n Active slot:{}", output, slot);
                    },
                    Err(e) => eprintln!("Convert Failed: {}", e),
                }
            }
        } 
    });
    return output;
}

/// Check if files matching pattern exist in the specified folder
/// 
/// # Parameters
/// - `dir_path`: Folder path (absolute/relative path)
/// - `pattern`: file pattern
/// 
/// # Returns
/// - `Result<bool, CheckFileError>`: Ok(true) if exists, Ok(false) if not found, Error if exception occurs
fn has_file_in_folder(dir_path: &str, glob_pattern: &str) -> Result<bool, CheckFileError> {
    // 1. Verify if the folder exists
    let dir = Path::new(dir_path);
    if !dir.exists() {
        return Err(CheckFileError::DirectoryNotFound);
    }
    if !dir.is_dir() {
        return Err(CheckFileError::InvalidPath);
    }

    // 2. Parse wildcard pattern and iterate matched files
    let entries = glob(&glob_pattern)
        .map_err(|e| CheckFileError::GlobError(e))?;

    for entry in entries {
        match entry {
            Ok(path) => {
                // Ensure the matched entry is a file (not a folder)
                if path.is_file() {
                    println!("Matched file found: {}", path.display());
                    return Ok(true);
                }
            }
            Err(e) => {
                return Err(CheckFileError::GlobIterError(e));
            }
        }
    }

    // No matched files found
    Ok(false)
}

pub fn check_necessary_files_in_edl_folder(path: &str) -> bool {
    if check_folder_exist(&path) {
        let meta_folder = format!("{}/META", path);
        if check_folder_exist(&meta_folder) {
            let super_define = format!("{}/super_def.*.json", meta_folder);
            match has_file_in_folder(&meta_folder, &super_define) {
                Ok(exists) => {
                    if exists {
                        println!("✅ Folder {} contains super_def.*.json files", &meta_folder);
                    } else {
                        println!("❌ No super_def.*.json files found in folder {}", &meta_folder);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Check failed: {}", e);
                }
            }
        } else {
            eprintln!("Folder not found:{}", &meta_folder);
            return false;
        }

        let img_folder = format!("{}/IMAGES", path);
        if check_folder_exist(&img_folder) {
            let rawprogram_xml = format!("{}/rawprogram?.xml", img_folder);
            match has_file_in_folder(&img_folder, &rawprogram_xml) {
                Ok(exists) => {
                    if exists {
                        println!("✅ Folder {} contains rawprogram?.xml files", &img_folder);
                    } else {
                        println!("❌ No rawprogram?.xml files found in folder {}", &img_folder);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Check failed: {}", e);
                }
            }

            let patch_xml = format!("{}/patch?.xml", img_folder);
            match has_file_in_folder(&img_folder, &patch_xml) {
                Ok(exists) => {
                    if exists {
                        println!("✅ Folder {} contains patch?.xml files", &img_folder);
                    } else {
                        println!("❌ No patch?.xml files found in folder {}", &img_folder);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Check failed: {}", e);
                }
            }
        } else {
            eprintln!("Folder not found:{}", &img_folder);
            return false;
        }
    } else {
        eprintln!("Folder not found:{}", &path);
        return false;
    }
    return true;
}