# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project called `script-parser` that parses enhanced Markdown files containing screenplay content and outputs formatted PDF documents. The project transforms a custom domain-specific language (DSL) for screenwriting into professional script layouts.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run example.md` - Build and run with example file
- `cargo run -- -o output.pdf example.md` - Run with custom output file
- `cargo run -- --validate-only example.md` - Validate script without generating PDF
- `cargo check` - Check code without building
- `cargo clippy` - Run linting checks
- `cargo fmt` - Format code according to Rust standards

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test

## Architecture

The project implements a complete screenplay parsing pipeline:

### Core Modules
- `src/main.rs` - CLI entry point with clap argument parsing
- `src/ast.rs` - Abstract Syntax Tree definitions (Script, Scene, ScriptElement)
- `src/lexer.rs` - Tokenizes markdown input into meaningful tokens
- `src/parser.rs` - Converts token stream into AST
- `src/renderer.rs` - Transforms AST into formatted PDF using printpdf

### Dependencies
- **nom** (7.1) - Parser combinators for lexical analysis
- **regex** (1.10) - Pattern matching for script elements
- **printpdf** (0.6) - PDF generation
- **clap** (4.4) - Command-line interface with derive features
- **anyhow** (1.0) - Error handling
- **serde** (1.0) - Data serialization with derive features

## Script Format Specification

### Structure
Input files consist of three sections:
1. **Title Section** - Free-form markdown content (starts with `#`)
2. **Character Map** - Character code definitions (starts with `## Characters`)
3. **Script Content** - Screenplay using character codes (starts with `## Script`)

### Format Elements
- `ABC: dialogue text` - Character dialogue
- `N: narration text` or plain text - Narrator/stage directions
- `(action description)` - Action/direction within dialogue
- `[Location Name]` - Scene location headers
- Character definitions: `CODE: Full Character Name`

## Current Implementation Status

✅ **Completed Features:**
- Basic lexer and parser for screenplay markdown
- AST generation with proper structure
- PDF rendering with basic formatting
- CLI interface with validation mode
- Character mapping system
- Support for dialogue, narration, actions, and locations

⚠️ **Known Issues:**
- Compiler warnings for unused imports and fields
- Limited PDF formatting options
- No error line/column reporting yet
- No configuration file support

## Future Development

### Phase 1 Enhancements
- Fix compiler warnings
- Add proper error reporting with line numbers
- Implement configuration file support
- Add comprehensive testing suite

### Phase 2 Features
- Multiple output formats (HTML, LaTeX)
- Advanced PDF formatting options
- Script validation rules
- Performance optimization

## Testing

Use `example.md` as a test file. It contains:
- 3 character definitions (ALICE, BOB, N)
- 1 scene with location [Living Room]
- Mix of dialogue, narration, and action elements