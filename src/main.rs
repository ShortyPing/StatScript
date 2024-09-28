use std::fs;
use std::process::exit;

use clap::Parser;

use stat_script::lexer::tokenizer::Tokenizer;
use stat_script::parse::parser::StatParser;
use stat_script::runtime;

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

    let mut program = runtime::program::Program::new(ast);
;
    
    match program.execute() {
        None => {}
        Some(err) => {
            eprintln!("A runtime error occurred: {}", err.message);
            exit(1)
        }
    };


}


