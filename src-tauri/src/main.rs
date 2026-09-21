// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// D&D Mate tauri backend entry.
fn main() {
    dnd_m8_lib::run()
}
