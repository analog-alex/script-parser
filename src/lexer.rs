use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    SectionHeader(String),
    CharacterDef { code: String, name: String },
    DialogueLine { speaker: String, text: String },
    NarrationLine(String),
    ActionText(String),
    LocationHeader(String),
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let lines: Vec<&str> = self.input.lines().collect();
        
        let mut current_section = "";
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            // Section headers
            if trimmed.starts_with("## ") {
                let section_name = trimmed[3..].trim();
                current_section = section_name;
                tokens.push(Token::SectionHeader(section_name.to_string()));
                continue;
            }
            
            // Title section (# header)
            if trimmed.starts_with("# ") {
                current_section = "title";
                tokens.push(Token::SectionHeader("title".to_string()));
                continue;
            }
            
            match current_section.to_lowercase().as_str() {
                "characters" => {
                    if let Some(token) = self.parse_character_def(trimmed) {
                        tokens.push(token);
                    }
                }
                "script" => {
                    if let Some(token) = self.parse_script_line(trimmed) {
                        tokens.push(token);
                    }
                }
                _ => {}
            }
        }
        
        tokens.push(Token::EOF);
        tokens
    }
    
    fn parse_character_def(&self, line: &str) -> Option<Token> {
        let re = Regex::new(r"^([A-Z]+):\s*(.+)$").unwrap();
        if let Some(captures) = re.captures(line) {
            let code = captures.get(1)?.as_str().to_string();
            let name = captures.get(2)?.as_str().to_string();
            return Some(Token::CharacterDef { code, name });
        }
        None
    }
    
    fn parse_script_line(&self, line: &str) -> Option<Token> {
        // Location header [Location Name]
        if line.starts_with('[') && line.ends_with(']') {
            let location = line[1..line.len()-1].to_string();
            return Some(Token::LocationHeader(location));
        }
        
        // Action text (action description)
        if line.starts_with('(') && line.ends_with(')') {
            let action = line[1..line.len()-1].to_string();
            return Some(Token::ActionText(action));
        }
        
        // Dialogue line ABC: dialogue text
        let re = Regex::new(r"^([A-Z]+):\s*(.+)$").unwrap();
        if let Some(captures) = re.captures(line) {
            let speaker = captures.get(1)?.as_str().to_string();
            let text = captures.get(2)?.as_str().to_string();
            return Some(Token::DialogueLine { speaker, text });
        }
        
        // Narration line (default)
        Some(Token::NarrationLine(line.to_string()))
    }
}