use clap::Parser;
use std::fs;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "cli here")]
struct Cli {
    query: String,
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    run(cli);
}

fn run(cli:Cli) {

    match fs::read_to_string(cli.path) {
        Ok(content) => {
            println!("{}",content);
        }
        Err(e)=>{
            println!("Error :{}", e.to_string().italic().red());
        }
    }

}
