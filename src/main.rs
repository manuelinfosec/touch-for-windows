use chrono::Local;
use clap::{arg, ArgAction, Parser};
use filetime::{set_file_atime, set_file_mtime, FileTime};
use regex::Regex;
use std::{fs::File, path::Path, process, time::SystemTime};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too."
)]
struct Command {
    // Flag to indicate changing the file access time
    #[arg(short, action = ArgAction::SetTrue, help = "change file access time")]
    access: bool,

    // Optional date string to use for setting the file time instead of the current time
    #[arg(
        short,
        long,
        help = "specifies the date to use instead of current time"
    )]
    date: Option<String>,

    // Flag to indicate changing the file modification time
    #[arg(short, action = ArgAction::SetTrue, help = "change file modification time")]
    modify: bool,

    // List of file paths to operate on
    file_paths: Vec<String>,
}

fn main() {
    // Parse command line arguments
    let args: Command = Command::parse();

    // Ensure at least one file path is provided
    if args.file_paths.len() == 0 {
        eprintln!("No filename(s) specified");
        process::exit(1);
    }

    // Regex pattern to validate Windows file naming rules
    let pattern: Regex = Regex::new(r#"^[^\\/:*?"<>|]{1,255}$"#).unwrap();

    // Iterate over each file path provided
    for file_path in &args.file_paths {
        // Check if file path matches Windows file naming rules
        if !pattern.is_match(&file_path) {
            eprintln!("The filename, directory name, or volume label syntax is incorrect.");
            process::exit(1);
        }

        // Create a Path object for the current file path
        let path: &Path = Path::new(file_path);

        // Check if the path already exists
        if path.exists() {
            eprintln!("A file {file_path} already exists");
            process::exit(1);
        }
        // If the file does not exist, create it
        else {
            File::create(path).unwrap_or_else(|_| -> File {
                eprintln!("Error occurred while creating file.");
                process::exit(1);
            });
        }

        // Determine the datetime to use for file timestamps
        let datetime: FileTime = if let Some(date) = &args.date {
            // Parse the date string into a chrono DateTime object
            let datetime: chrono::DateTime<chrono::Utc> =
                dateparser::parse_with_timezone(&date, &Local)
                    // Handle parsing failure by exiting the program with an error message
                    .unwrap_or_else(|_| -> chrono::DateTime<chrono::Utc> {
                        eprintln!("The time format is incorrect");
                        process::exit(1);
                    });
            // Convert parsed DateTime to FileTime
            FileTime::from_system_time(SystemTime::from(datetime))
        }
        // If no date is supplied, use the current system time
        else {
            FileTime::from_system_time(SystemTime::now())
        };

        // If the access flag is set, update the file's access time
        if args.access {
            set_file_atime(path, datetime).unwrap_or_else(|_| {
                eprintln!("Cannot set last access time");
                process::exit(1);
            });
        }

        // If the modify flag is set, update the file's modification time
        if args.modify {
            set_file_mtime(path, datetime).unwrap_or_else(|_| {
                eprintln!("Cannot set last modify time");
                process::exit(1);
            });
        }
    }
}
