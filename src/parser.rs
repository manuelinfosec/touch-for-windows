use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "DESCRIPTION")]
struct Command {
    pub file_paths: Vec<String>,
}
