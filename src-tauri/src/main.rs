// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cards_functions;
mod global_variables;

use cards_functions::*;
use global_variables::CARD_COUNT; // Import CARD_COUNT from global_variables // Import cards_add_list from cards_functions

#[tauri::command]
fn card_count() -> Result<usize, String> {
    // Retrieve the card count from the global variable
    Ok(*CARD_COUNT)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            card_count,
            cards_add_list,
            card_details,
            questions_and_answer,
            save_questions_and_answer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
