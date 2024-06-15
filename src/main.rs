use chrono::Local;
use clap::{arg, ArgAction, Parser};
use filetime::FileTime;
use regex::Regex;
use std::{path::Path, process, time::SystemTime};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too."
)]
struct Command {
    #[arg(short, action = ArgAction::SetTrue, help = "change file access time")]
    access: bool,

    #[arg(
        short,
        long,
        help = "specifies the date to use instead of current time"
    )]
    date: Option<String>,

    #[arg(short, action = ArgAction::SetTrue, help = "change file modification time")]
    modify: bool,

    file_paths: Vec<String>,
}

fn main() {
    // Parse command line arguments with the pattt
    let args: Command = Command::parse();

    // Check if file paths exists in the arguments
    if args.file_paths.len() == 0 {
        eprintln!("No filename(s) specified");
        process::exit(1);
    }

    // Regex for Windows filenames
    let pattern: Regex = Regex::new(r#"""^(?!^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(\..*)?$)([^\\/:*?"<>|][^\\/:*?"<>|]{0,254}[^\\/:*?"<>|.\s])$"#).unwrap();

    // Iterate through file paths
    for file_path in &args.file_paths {
        // Check if file path matches Windows file naming rules
        if !pattern.is_match(&file_path) {
            eprintln!("The filename, directory name, or volume label syntax is incorrect.");
            process::exit(1);
        }

        // Create a Path object for file system operations
        let path: &Path = Path::new(file_path);

        // Check if the path already exists
        if path.exists() {
            eprintln!("A file {file_path} already exists");
            process::exit(1);
        }

        match args.date {
            Some(date) => { 
                let datetime: chrono::DateTime<chrono::Utc> = dateparser::parse_with_timezone(&date, &Local).unwrap_or_else(|_| {
                    eprintln!("The time format is incorrect");
                    process::exit(1);
                });
            }
            None => { 
                let time_now: FileTime = FileTime::from_system_time(SystemTime::now())
            }
        }
    }
}
