// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = drasus_desktop_lib::run() {
        eprintln!("No se pudo iniciar Drasus Desktop: {error}");
        std::process::exit(1);
    }
}
