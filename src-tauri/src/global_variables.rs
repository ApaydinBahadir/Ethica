// src/global_variables.rs

use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use dirs::home_dir;

lazy_static::lazy_static! {
    pub static ref CONFIG_DIRECTORY: String = {
        // Get the home directory
        let home = home_dir().expect("Could not determine home directory");
        
        // Construct the path to the .config/Ethica/config.txt file
        let config_path: PathBuf = [home.to_str().unwrap(), ".config", "Ethica", "config.txt"].iter().collect();

        // Check if the config file exists
        if !config_path.exists() {
            panic!("Config file not found");
        }

        // Read the directory path from the config file
        let file = File::open(&config_path).expect("Failed to open config file");
        let reader = io::BufReader::new(file);
        let mut directory = String::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line from config file");
            if line.starts_with("directory") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    directory = parts[1].trim().to_string();
                    break;
                }
            }
        }

        if directory.is_empty() {
            panic!("Directory path not found in config file");
        }

        directory
    };

    pub static ref CARD_COUNT: usize = {
      // Retrieve the directory path from the global variable
      let directory = &*CONFIG_DIRECTORY;

      // Check if the directory exists
      let path = Path::new(&directory);
      if !path.exists() {
          panic!("Directory specified in config not found");
      }

      // Read the directory and count the files matching the pattern
      match fs::read_dir(path) {
          Ok(entries) => {
              entries
                  .filter_map(Result::ok)
                  .filter(|entry| {
                      let file_name = entry.file_name();
                      let file_name_str = file_name.to_string_lossy();
                      file_name_str.ends_with(".card")
                  })
                  .count()
          }
          Err(e) => panic!("Failed to read directory: {}", e),
      }
  };
}