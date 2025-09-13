mod ast;
mod debug;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod ui;
mod utils;

use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::io::{self, Write};

use debug::DebugInfo;
use errors::funny_errors::FunnyErrorGenerator;
use errors::hinglish_errors::HinglishErrorGenerator;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use ui::*;
use utils::colors::PaneerColors;

use crate::utils::syntax_highlighter::print_code_block;

/// Main entry point for the PaneerLang interpreter
/// Handles command line arguments and routes to appropriate execution mode
fn main() {
    // Force colors to be enabled for syntax highlighting
    colored::control::set_override(true);

    let matches = Command::new("paneerlang")
        .version("0.1.0")
        .author("Paneer Lang Team")
        .about("A programming language with Hindi-inspired syntax")
        .arg(
            Arg::new("file")
                .help("The .paneer file to execute")
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::new("repl")
                .short('r')
                .long("repl")
                .help("Start interactive REPL mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode with detailed output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("repl") {
        start_repl();
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        let debug = matches.get_flag("debug");
        run_file(file_path, debug);
    } else {
        print_error_banner();
        print_usage();
        std::process::exit(1);
    }
}

/// Executes a PaneerLang file from the filesystem
///
/// # Arguments
/// * `file_path` - Path to the .paneer file to execute
/// * `debug` - Whether to enable debug output with syntax highlighting
fn run_file(file_path: &str, debug: bool) {
    print_file_info(file_path, debug);

    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            let error_gen = HinglishErrorGenerator::new();
            let hinglish_error = error_gen.format_hinglish_error(
                &format!("Could not read file: {}", err),
                Some(file_path),
                None,
            );
            eprintln!("{}", hinglish_error);
            std::process::exit(1);
        }
    };

    if debug {
        print_debug_info(&source);
        print_code_block("Source Code with Syntax Highlighting:", &source);
    }

    print_execution_start();

    if let Err(err) = execute(&source, debug) {
        println!("{}", PaneerColors::separator(&"─".repeat(60)));

        let error_gen = HinglishErrorGenerator::new();
        let hinglish_error =
            error_gen.format_hinglish_error(&err.to_string(), Some(file_path), None);
        eprintln!("{}", hinglish_error);
        std::process::exit(1);
    }

    let error_gen = FunnyErrorGenerator::new();
    println!("{}", "─".repeat(60).bright_black());
    println!("{}", error_gen.format_success_message());
}

/// Starts the interactive REPL (Read-Eval-Print Loop) mode
/// Allows users to execute PaneerLang statements interactively
fn start_repl() {
    print_repl_banner();

    let mut interpreter = Interpreter::new();

    loop {
        print!("{} ", "paneer>".blue().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "exit" {
                    println!("{}", "Goodbye!".green());
                    break;
                }

                if input == "help" {
                    print_help();
                    continue;
                }

                // For REPL, execute single statements with error handling
                if let Err(err) = execute_repl(&mut interpreter, input) {
                    let error_gen = FunnyErrorGenerator::new();
                    let error_type = if err.to_string().contains("type") {
                        "type"
                    } else if err.to_string().contains("Undefined") {
                        "undefined"
                    } else if err.to_string().contains("Expected") {
                        "syntax"
                    } else {
                        "general"
                    };

                    let funny_error =
                        error_gen.format_error(error_type, &err.to_string(), None, None);
                    eprintln!("{}", funny_error);
                }
            }
            Err(err) => {
                eprintln!("{} {}", "Error reading input:".red(), err);
                break;
            }
        }
    }
}

/// Executes PaneerLang source code through the complete compilation pipeline
///
/// # Arguments
/// * `source` - The PaneerLang source code to execute
/// * `debug` - Whether to enable debug output showing compilation phases
///
/// # Returns
/// * `Ok(())` if execution succeeds
/// * `Err(Box<dyn std::error::Error>)` if any phase fails
fn execute(source: &str, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let debug_info = DebugInfo::new(debug);
    let start_time = std::time::Instant::now();

    // Phase 1: Lexical Analysis
    debug_info.print_phase("Lexical Analysis");
    debug_info.print_lexer_info(source);
    let lexer = Lexer::new(source)?;

    // Phase 2: Parsing
    debug_info.print_phase("Syntax Analysis");
    let mut parser = Parser::new(lexer);
    let program = parser.parse()?;
    debug_info.print_parser_info(true, Some(program.statements.len()));
    debug_info.print_ast_structure(&program);

    // Phase 3: Interpretation
    debug_info.print_phase("Code Execution");
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(program);

    let duration = start_time.elapsed();

    match result {
        Ok(_) => {
            debug_info.print_interpreter_info(true);
            debug_info.print_execution_summary(true, Some(duration));
            Ok(())
        }
        Err(e) => {
            debug_info.print_interpreter_info(false);
            debug_info.print_execution_summary(false, Some(duration));
            Err(e.into())
        }
    }
}

/// Executes a single statement or expression in REPL mode
///
/// # Arguments
/// * `interpreter` - Mutable reference to the interpreter instance
/// * `input` - The user input to execute
///
/// # Returns
/// * `Ok(())` if execution succeeds
/// * `Err(Box<dyn std::error::Error>)` if parsing or execution fails
fn execute_repl(
    interpreter: &mut Interpreter,
    input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Add semicolon if not present for single expressions
    let input = if !input.ends_with(';') && !input.contains('{') {
        format!("{};", input)
    } else {
        input.to_string()
    };

    let lexer = Lexer::new(&input)?;
    let mut parser = Parser::new(lexer);
    let program = parser.parse()?;

    interpreter.interpret(program)?;

    Ok(())
}
