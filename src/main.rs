pub mod ast;
pub mod lexer;
pub mod parser;
pub mod renderer;
pub mod validator;

use clap::{Arg, Command};
use std::fs;
use anyhow::Result;

use lexer::Lexer;
use parser::Parser;
use renderer::PdfRenderer;
use validator::Validator;

use log::{info, debug};

fn main() -> Result<()> {
    let matches = Command::new("script-parser")
        .version(env!("CARGO_PKG_VERSION"))
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

    info!("Reading input file: {}", input_file);
    let content = fs::read_to_string(input_file)?;

    info!("Tokenizing...");
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    debug!("Generated {} tokens", tokens.len());

    info!("Parsing...");
    let mut parser = Parser::new(tokens);
    let script = parser.parse()?;

    debug!("Script parsed successfully! Title section: {}, Characters: {}, Scenes: {}",
        if script.title_section.is_empty() { "empty" } else { "present" },
        script.characters.len(),
        script.scenes.len()
    );

    info!("Validating script...");
    let mut validator = Validator::new();
    validator.validate(&script)?;

    debug!("Script validation completed successfully!");

    if validate_only {
        info!("Validation complete. No PDF generated.");
        return Ok(());
    }

    info!("Generating PDF: {}", output_file);
    let renderer = PdfRenderer::new();
    renderer.render(&script, output_file)?;

    info!("PDF generated successfully!");
    Ok(())
}
