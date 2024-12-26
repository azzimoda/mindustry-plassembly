use std::env;
use std::fs;
use std::process;

mod lexer;
mod processor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mdll <file>");
        process::exit(1);
    }

    let file_path = &args[1];
    let output_file_path = format!("{}.txt", file_path);

    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("No such file: {}", file_path);
            process::exit(1);
        }
    };

    let tokens = lexer::Lexer::new(&source).all_lines();
    // dbg!(&tokens);
    let mut processor = processor::MacroProcessor::new(tokens);
    let result = processor::stringify_tokens(processor.run());

    fs::write(output_file_path.clone(), result).expect("Unable to write data");
    println!("Output was saved in file `{output_file_path}`.")
}
