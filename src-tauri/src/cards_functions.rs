// src/cards_functions.rs

use crate::global_variables; // Import global_variables module

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
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
pub fn update_questions_and_answer(
    file_name: String,
    data: HashMap<String, String>,
    number_of_question: u64,
) -> Result<bool, String> {
    let mut count: u64 = 1;

    let file_path_string: String =
        global_variables::CONFIG_DIRECTORY.to_string() + &file_name + ".card";

    let file_path: &Path = Path::new(&file_path_string);

    let file: File = File::open(&file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path_string, e))?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|line: Result<String, io::Error>| {
            line.map_err(|e: io::Error| format!("Failed to read line from file: {}", e))
        })
        .collect::<Result<_, _>>()?;

    let mut i: usize = 0;
    while i < lines.len() {
        if lines[i].starts_with("#Question=") {
            if count == number_of_question {
                lines[i] = format!("#Question={}", data["question"]);
                if i + 1 < lines.len() {
                    lines[i + 1] = format!("#Answer={}", data["answer"]);
                } else {
                    return Err("No answer found for the question".to_string());
                }
                break;
            }
            count += 1;
        }
        i += 1;
    }

    let mut file: File = File::create(&file_path)
        .map_err(|e: io::Error| format!("Failed to create file {}: {}", file_path_string, e))?;

    for line in lines {
        writeln!(file, "{}", line).map_err(|e| format!("Failed to write line to file: {}", e))?;
    }

    Ok(true)
}

#[tauri::command]
pub fn add_questions_and_answer(
    file_name: String,
    data: HashMap<i32, HashMap<String, String>>,
) -> Result<(), String> {
    // Define the path to the file
    let file_path_string: String =
        global_variables::CONFIG_DIRECTORY.to_string() + &file_name + ".card";
    let file_path: &Path = Path::new(&file_path_string);

    // Open the file in append mode, create if it doesn't exist
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| format!("Failed to open or create file {}: {}", file_path_string, e))?;

    let mut writer = BufWriter::new(file);

    // Convert HashMap to BTreeMap to ensure the data is sorted by key
    let sorted_data: BTreeMap<_, _> = data.into_iter().collect();

    // Iterate over the sorted data and write to the file
    for (_key, value) in &sorted_data {
        writeln!(
            writer,
            "\n#Question={}",
            value.get("question").unwrap_or(&String::new())
        )
        .map_err(|e| format!("Failed to write to file {}: {}", file_path_string, e))?;
        writeln!(
            writer,
            "#Answer={}",
            value.get("answer").unwrap_or(&String::new())
        )
        .map_err(|e| format!("Failed to write to file {}: {}", file_path_string, e))?;
    }

    // Flush writer to ensure all data is written to file
    writer
        .flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))?;

    // Modify the file content to update the count of questions
    modify_no_question_content(&file_path, sorted_data.len() as i32)?;

    Ok(())
}

#[tauri::command]
pub fn remove_question_answer(
    filename: String,
    data: HashMap<String, String>,
) -> Result<(), String> {
    // Construct the full file path
    let file_path_string: String =
        global_variables::CONFIG_DIRECTORY.to_string() + &filename + ".card";
    let file_path = Path::new(&file_path_string);

    // Print the file path and question for debugging
    println!("{}", file_path_string);
    println!("{:?}", data.get("question"));

    println!("_____");

    // Read the file content
    let file_content =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Collect lines from the file content
    let lines: Vec<&str> = file_content.lines().collect();

    // Check if the question and answer exist in the data HashMap
    if let (Some(question), Some(answer)) = (data.get("question"), data.get("answer")) {
        // Filter out the lines that match the question or answer
        let filtered_lines: Vec<&str> = lines
            .into_iter()
            .filter(|&line| {
                !(line.starts_with("#Question=") && line.contains(question))
                    && !(line.starts_with("#Answer=") && line.contains(answer))
            })
            .collect();

        // Write the filtered lines back to the file
        let new_content = filtered_lines.join("\n");
        fs::write(&file_path, new_content)
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        println!(
            "Question '{}' and answer '{}' removed from the file.",
            question, answer
        );
    } else {
        return Err("Question or answer not found in data".to_string());
    }

    modify_no_question_content(file_path, -1)?;

    Ok(())
}

// Function to modify file content
fn modify_no_question_content(file_path: &Path, new_count: i32) -> Result<(), String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path.display(), e))?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Modify the line containing the count of questions
    let count_line_index: Option<usize> = lines
        .iter()
        .position(|line: &String| line.contains("Count Of Questions="));

    if let Some(index) = count_line_index {
        // Extract current count from the line
        let mut count_line: String = lines[index].clone();
        let current_count: i32 = count_line
            .chars()
            .skip_while(|&c| !c.is_digit(10))
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);

        let updated_count = current_count + new_count;
        count_line = count_line.replacen(&current_count.to_string(), &updated_count.to_string(), 1);

        // Update the line in the vector of lines
        lines[index] = count_line;
    } else {
        // If count line not found, add it to the end of the file
        lines.push(format!("Count Of Questions={}", new_count));
    }

    // Write the modified content back to the file
    let mut file = File::create(file_path)
        .map_err(|e| format!("Failed to create file {}: {}", file_path.display(), e))?;
    for line in lines {
        writeln!(file, "{}", line)
            .map_err(|e| format!("Failed to write to file {}: {}", file_path.display(), e))?;
    }

    Ok(())
}
