use std::fs;

use clap::Parser;
use stat_script::lexer::tokenizer::Tokenizer;
use std::process::exit;
use stat_script::parse::parser::StatParser;

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

    let tokenizer = Tokenizer::new(file_content);

    let mut parser = StatParser::new(tokenizer);

    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to parse at position {}: {}", e.position, e.message);
            exit(1)
        }
    };

    println!("{:#?}", ast);

}

