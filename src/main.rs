// Remove node_modules from the child directories which is passed by CLI argument

use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => search_child_dirs(&args[1]),
        _ => process::exit(1)
    }
}

fn search_child_dirs(dir: &String) {
    if let Ok(paths) = fs::read_dir(dir) {
        for entry_result in paths {
            if let Ok(entry) = entry_result {
                if entry.path().is_dir() {
                    if let Ok(dirname) = entry.path().into_os_string().into_string() {
                        remove_node_modules(&dirname)
                    }
                }
            }
        }
    }
}

fn remove_node_modules(dir: &String) {
    if let Ok(paths) = fs::read_dir(dir) {
        for entry_result in paths {
            if let Ok(entry) = entry_result {
                if entry.path().is_dir() && entry.path().ends_with("node_modules") {
                    let result = fs::remove_dir_all(entry.path());
                    match result {
                        Ok(_n) => println!("Removed:{:?}", entry.path()),
                        Err(err) => println!("Error:{:?}", err)
                    }
                }
            }
        }
    }
}
