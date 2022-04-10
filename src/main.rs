#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

mod app;
mod obd;

use std::{env, error::Error, process};

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).ok_or("csv file argument required")?;
    let csv_log = obd::CsvLog::from_file(file_path)?;

    let app = app::App::new(csv_log);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
