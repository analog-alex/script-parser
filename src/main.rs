pub mod ast;
pub mod lexer;
pub mod parser;
pub mod renderer;

use clap::{Arg, Command};
use std::fs;
use anyhow::Result;

use lexer::Lexer;
use parser::Parser;
use renderer::PdfRenderer;

fn main() -> Result<()> {
    let matches = Command::new("script-parser")
        .version("0.1.0")
        .about("Parses screenplay markdown files and generates PDF output")
        .arg(
            Arg::new("input")
                .help("Input markdown file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output PDF file")
                .default_value("output.pdf"),
        )
        .arg(
            Arg::new("validate-only")
                .short('v')
                .long("validate-only")
                .help("Only validate, don't generate PDF")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let validate_only = matches.get_flag("validate-only");

    println!("Reading input file: {}", input_file);
    let content = fs::read_to_string(input_file)?;

    println!("Tokenizing...");
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    println!("Generated {} tokens", tokens.len());

    println!("Parsing...");
    let mut parser = Parser::new(tokens);
    let script = parser.parse()?;

    println!("Script parsed successfully!");
    println!("  - Title section: {}", if script.title_section.is_empty() { "empty" } else { "present" });
    println!("  - Characters: {}", script.characters.len());
    println!("  - Scenes: {}", script.scenes.len());

    if validate_only {
        println!("Validation complete. No PDF generated.");
        return Ok(());
    }

    println!("Generating PDF: {}", output_file);
    let renderer = PdfRenderer::new();
    renderer.render(&script, output_file)?;

    println!("PDF generated successfully!");
    Ok(())
}
