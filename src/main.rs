//!Tiny Language Parser CLI

use clap::{Parser, Subcommand};
use std::fs;
use tiny_lang_parser::{parse_program, Interpreter};

#[derive(Parser)]
#[command(name = "tiny-lang-parser")]
#[command(about = "A parser and interpreter for Tiny Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Parse and execute a Tiny Language file
    Parse {
        ///Path to the file to parse
        file: String,
    },
    ///Display help information
    ParserHelp,
    ///Display credits and authorship information  
    Credits,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => {
            let content = fs::read_to_string(&file)
                .map_err(|e| format!("Failed to read file {}: {}", file, e))?;

            println!("Parsing file: {}", file);
            println!("Source code:\n{}", content);

            let ast = parse_program(&content).map_err(|e| format!("Parse error: {}", e))?;

            println!("\nAST: {:#?}", ast);

            let mut interpreter = Interpreter::new();
            interpreter
                .eval(&ast)
                .map_err(|e| format!("Evaluation error: {}", e))?;

            println!("\nExecution completed.");
            println!("Variables: {:?}", interpreter.variables);
        }
        Commands::ParserHelp => {
            print_help();
        }
        Commands::Credits => {
            print_credits();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Tiny Language Parser");
    println!();
    println!("USAGE:");
    println!("    tiny-lang-parser <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    parse <file>    Parse and execute a Tiny Language file");
    println!("    help            Display this help message");
    println!("    credits         Display credits and authorship information");
    println!();
    println!("Tiny Language Grammar:");
    println!("    program     = {{ statement* }}");
    println!("    statement   = {{ (assignment | expression) \";\" }}");
    println!("    assignment  = {{ identifier \"=\" expression }}");
    println!("    expression  = {{ term (add_op term)* }}");
    println!("    term        = {{ factor (mul_op factor)* }}");
    println!("    factor      = {{ number | identifier | \"(\" expression \")\" }}");
    println!("    add_op      = {{ \"+\" | \"-\" }}");
    println!("    mul_op      = {{ \"*\" | \"/\" }}");
    println!("    number      = {{ ASCII_DIGIT+ }}");
    println!("    identifier  = {{ ASCII_ALPHA_LOWER+ }}");
}

fn print_credits() {
    println!("Tiny Language Parser");
    println!("Made by Rublevskyi Orest");
    println!("Created as an educational project on a rust course 2025");
    println!();
    println!("Features:");
    println!("  - Parser for a simple language with variables and arithmetic");
    println!("  - AST generation");
    println!("  - Interpreter with variable storage");
    println!("  - Error handling");
    println!("  - Unit test coverage");
    println!();
    println!("Built with:");
    println!("  - Rust Programming Language");
    println!("  - Pest parser generator");
    println!("  - Clap for command-line interface");
}
