// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::panic;
use std::path::PathBuf;

fn install_panic_handler() {
    panic::set_hook(Box::new(|info| {
        let filename = format!("error_{}.txt", Local::now().format("%d-%m-%Y %H-%M-%S"));

        let mut path = PathBuf::from("errors");
        let _ = fs::create_dir(&path);
        path.push(filename);

        let location = info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());

        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<dyn Any>".to_string()
        };

        let text = format!(
            "Panic at: {}\nLocation: {}\nMessage: {}",
            Local::now().format("%d-%m-%Y %H:%M:%S"),
            location,
            payload
        );

        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(text.as_bytes());
            let _ = file.flush();
        }

        eprint!("{}\n", text);
    }));
}

fn main() {
    install_panic_handler();

    bubble_lib::run();
}
