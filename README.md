# Script Parser

A Rust application that parses enhanced Markdown files containing screenplay content and outputs formatted PDF documents. The tool transforms a custom domain-specific language (DSL) for screenwriting into professional script layouts.

## Features

- **Markdown-based screenplay format** - Write scripts in an enhanced Markdown syntax
- **Character mapping** - Define character codes for cleaner script writing
- **PDF generation** - Professional screenplay formatting with proper margins and fonts
- **Validation mode** - Check script syntax without generating output
- **CLI interface** - Easy-to-use command-line tool

## Installation

```bash
git clone <repository-url>
cd script-parser
cargo build --release
```

## Usage

### Basic Usage

```bash
# Parse a script and generate PDF
cargo run input.md

# Specify custom output file
cargo run -- -o my_script.pdf input.md

# Validate script syntax only
cargo run -- --validate-only input.md
```

### Command Line Options

```
script-parser [OPTIONS] <INPUT_FILE>

Options:
  -o, --output <FILE>     Output PDF file [default: output.pdf]
  -v, --validate-only     Only validate, don't generate PDF
  -h, --help             Print help information
  -V, --version          Print version information
```

## Script Format

### File Structure

Your screenplay file should have three main sections:

```markdown
# Title Section
[Free form content - title, author, etc.]

## Characters
[Character code mappings]

## Script
[The actual screenplay content]
```

### Character Definitions

Define character codes in the Characters section:

```markdown
## Characters

ALICE: Alice Johnson
BOB: Bob Smith
N: Narrator
```

### Script Elements

#### Dialogue
```markdown
ALICE: I can't believe you're actually leaving.
BOB: I don't have a choice, Alice.
```

#### Narration
```markdown
N: The silence stretches between them, heavy with unspoken words.
```

Or without the N prefix:
```markdown
The lights are dim, casting long shadows across the room.
```

#### Action/Stage Directions
```markdown
(Alice turns away, looking out the window)
(Bob reaches for Alice's hand)
```

#### Scene Locations
```markdown
[Living Room]
[Kitchen - Morning]
```

## Complete Example

```markdown
# Sample Script

This is a sample screenplay for testing the script parser.

## Characters

ALICE: Alice Johnson
BOB: Bob Smith  
N: Narrator

## Script

[Living Room]

The lights are dim, casting long shadows across the room.

ALICE: I can't believe you're actually leaving.

BOB: I don't have a choice, Alice. The job in New York is too good to pass up.

(Alice turns away, looking out the window)

ALICE: What about us? What about everything we've built here?

N: The silence stretches between them, heavy with unspoken words.

BOB: We can make this work. Long distance relationships aren't impossible.

(Bob reaches for Alice's hand)

ALICE: Aren't they?
```

## Technical Details

### Dependencies
- **nom** - Parser combinators for lexical analysis
- **regex** - Pattern matching for script elements
- **printpdf** - PDF generation
- **clap** - Command-line interface
- **anyhow** - Error handling
- **serde** - Data serialization

### Architecture
- **Lexer** - Tokenizes input markdown into meaningful tokens
- **Parser** - Converts token stream into Abstract Syntax Tree (AST)
- **Renderer** - Transforms AST into formatted PDF output
- **CLI** - Command-line interface handling

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo clippy
cargo fmt
```

### Running with Example
```bash
cargo run example.md
```

## Future Enhancements

- Multiple output formats (HTML, LaTeX)
- Advanced formatting options
- Configuration file support
- Syntax highlighting for editors
- Live preview mode
- Script statistics and analysis

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.