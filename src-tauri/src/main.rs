// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! from {}", name, env::consts::OS)
}

// This is another commands that lists the files from a given directory
#[tauri::command]
fn list_files(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut v: Vec<String> = Vec::new();
    for path in paths {
        v.push(format!("{}", path.unwrap().path().display()))
    }
    return v;
}

// Here we create a tauri app specifying what functions we would like to expose
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
