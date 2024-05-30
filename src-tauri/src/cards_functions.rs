// src/cards_functions.rs

use crate::global_variables; // Import global_variables module

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

#[tauri::command]
pub fn cards_add_list() -> Result<Vec<String>, String> {
    // Access the config directory from the global_variables module
    let config_directory = &global_variables::CONFIG_DIRECTORY;

    // Convert the String config_directory into a PathBuf
    let directory_path = PathBuf::from(config_directory.to_string());

    // Check if the directory exists
    if !directory_path.exists() {
        return Err("Config directory does not exist".into());
    }

    // Read the contents of the directory
    match fs::read_dir(directory_path) {
        Ok(entries) => {
            // Collect the file names (without extension) into a vector of strings
            let file_names: Vec<String> = entries
                .filter_map(Result::ok) // Filter out any errors
                .filter(|entry| entry.path().is_file()) // Filter out only files
                .filter_map(|entry| {
                    entry
                        .path()
                        .file_stem()
                        .map(|stem| stem.to_string_lossy().into_owned())
                }) // Get the stem (file name without extension) and convert to owned String
                .collect();

            Ok(file_names)
        }

        Err(e) => Err(format!("Failed to read directory: {}", e)),
    }
}

#[tauri::command]
pub fn card_details(file_name: String) -> Result<HashMap<String, String>, String> {
    let mut details: HashMap<String, String> = HashMap::new();
    let mut flag: bool = false;

    let file_path_string: String =
        global_variables::CONFIG_DIRECTORY.to_string() + &file_name + ".card";
    let file_path: &Path = Path::new(&file_path_string);

    let file: File = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.map_err(|e| format!("Failed to read line from file: {}", e))?;

        if line.starts_with("#Details#") {
            flag = true;
        }

        if line.starts_with("#Details_End#") {
            flag = false;
        }

        if line.starts_with("Count Of Questions") && flag {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                details.insert(
                    "Count Of Questions".to_string(),
                    parts[1].trim().to_string(),
                );
                break;
            }
        }
    }

    // Return the details HashMap
    Ok(details)
}

#[tauri::command]
pub fn questions_and_answer(
    file_name: String,
) -> Result<HashMap<usize, HashMap<String, String>>, String> {
    let mut questions_answer: HashMap<usize, HashMap<String, String>> = HashMap::new();
    let mut count: usize = 1;

    let file_path_string: String =
        global_variables::CONFIG_DIRECTORY.to_string() + &file_name + ".card";
    let file_path: &Path = Path::new(&file_path_string);

    let file: File = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.map_err(|e| format!("Failed to read line from file: {}", e)))
        .collect::<Result<_, _>>()?;

    let mut i: usize = 0;
    while i < lines.len() {
        let line: &String = &lines[i];

        if line.starts_with("#Question=") {
            let question: String = line.trim_start_matches("#Question=").trim().to_string();
            let mut qa_map = HashMap::new();
            qa_map.insert("question".to_string(), question.clone());

            // Move to the next line to check for the answer
            if i + 1 < lines.len() && lines[i + 1].starts_with("#Answer=") {
                let answer_line: &String = &lines[i + 1];
                let answer: String = answer_line
                    .trim_start_matches("#Answer=")
                    .trim()
                    .to_string();
                qa_map.insert("answer".to_string(), answer);
                // Skip the answer line in the next iteration
                i += 1;
            }
            questions_answer.insert(count, qa_map);
            count += 1;
        }
        i += 1;
    }

    Ok(questions_answer)
}

#[tauri::command]
pub fn save_questions_and_answer(file_name: String, data: String) -> Result<String, String> {
    let path: String = global_variables::CONFIG_DIRECTORY.to_string() + &file_name + ".card";
    let result: Result<(), io::Error> = fs::write(&path, data);
    match result {
        Ok(_) => Ok("File saved successfully".to_string()),
        Err(e) => Err(format!("Failed to save file: {}", e)),
    }
}