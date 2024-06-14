use clap::{arg, ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too."
)]
pub struct Command {
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

    pub file_paths: Vec<String>,
}
