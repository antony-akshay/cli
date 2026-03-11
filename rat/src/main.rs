use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "cli here")]
struct Cli {
    #[arg(short, long)]
    number: bool,
    query: String,
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    run(cli);
}

fn run(cli: Cli) {
    match fs::read_to_string(cli.path) {
        Ok(content) => {
            if cli.number {
                for (i, line) in content.lines().enumerate() {
                    println!("{:>4} {}", i + 1, line);
                }
            } else {
                println!("{}", content);
            }
        }
        Err(e) => {
            println!("Error :{}", e.to_string().italic().red());
        }
    }
}
