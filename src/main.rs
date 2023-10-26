use std::fs;

use clap::Parser;
use stat_script::lexer::tokenizer::Tokenizer;
use std::process::exit;

#[derive(Debug, Parser)]
struct CLI {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let arguments = CLI::parse();

    let file_content = match fs::read_to_string(&arguments.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read file: \"{}\": {e}", arguments.file);
            exit(1)
        }
    };

    let mut tokenizer = Tokenizer::new(file_content);

    println!("{:?}", tokenizer.next_token());
    println!("{:?}", tokenizer.next_token());
}
