use arboard::Clipboard;
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
    #[arg(long)]
    clip: bool,
}

fn main() {
    let cli = Cli::parse();

    run(cli);
}

fn run(cli: Cli) {
    match fs::read_to_string(cli.path) {
        Ok(content) => {
            let mut output = String::new();

            if cli.number {
                for (i, line) in content.lines().enumerate() {
                    println!("{:>4} {}", i + 1, line);
                    output.push_str(&format!("{:>4} {}", i + 1, line));
                }
            } else {
                output = content;
            }

            if cli.clip {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(output.clone()).unwrap();
                println!("{}","copied to clipboard".green());   
            }else{
                println!("{}",output);
            }
        }
        Err(e) => {
            println!("Error :{}", e.to_string().italic().red());
        }
    }
}
