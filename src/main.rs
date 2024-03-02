mod file;
mod directory;

use clap::{App, Arg};
use crate::directory::FileOperation::*;

fn main() {
    let matches = App::new("File Management CLI")
        .version("1.0")
        .author("Mohammed Ali Zeeshan")
        .about("A simple file management system with CLI")
        .subcommand(App::new("list").about("List all files in the directory"))
        .subcommand(
            App::new("create")
                .about("Create a new file")
                .arg(Arg::with_name("filename").required(true).index(1)),
        )
        .subcommand(
            App::new("delete")
                .about("Delete a file")
                .arg(Arg::with_name("filename").required(true).index(1)),
        )
        .get_matches();

    let mut directory: Directory<u32> = Directory::new_directory();

    match matches.subcommand() {
        ("list", _) => directory.display_directory(),
        ("create", Some(submatches)) => {
            let filename = submatches.value_of("filename").unwrap();
            let new_file = file::FileManager::file {
                fileName: filename.to_string(),
                fileSize: 0,
                timestamp: "0".to_string(),
            };
            directory.create_file(filename.to_string(), new_file);
        }
        ("delete", Some(submatches)) => {
            let filename = submatches.value_of("filename").unwrap();
            match directory.delete_file(filename.to_string()) {
                Ok(_) => println!("File {} deleted successfully.", filename),
                Err(err) => println!("Error: {}", err),
            }
        }
        _ => println!("Invalid command. Use 'list', 'create', or 'delete'."),
    }
}