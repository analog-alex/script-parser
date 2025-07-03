use crate::ast::{Script, Scene, ScriptElement};
use crate::lexer::Token;
use anyhow::Result;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Script> {
        let mut script = Script::new();
        
        while !self.is_at_end() {
            match self.current_token() {
                Token::SectionHeader(section) => {
                    match section.to_lowercase().as_str() {
                        "title" => {
                            script.title_section = "Title Section".to_string();
                            self.advance();
                        }
                        "characters" => {
                            script.characters = self.parse_characters()?;
                        }
                        "script" => {
                            script.scenes = self.parse_script()?;
                        }
                        _ => {
                            self.advance();
                        }
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        Ok(script)
    }
    
    fn parse_characters(&mut self) -> Result<HashMap<String, String>> {
        let mut characters = HashMap::new();
        self.advance(); // Skip the "Characters" header
        
        while !self.is_at_end() {
            match self.current_token() {
                Token::CharacterDef { code, name } => {
                    characters.insert(code.clone(), name.clone());
                    self.advance();
                }
                Token::SectionHeader(_) => break,
                _ => {
                    self.advance();
                }
            }
        }
        
        Ok(characters)
    }
    
    fn parse_script(&mut self) -> Result<Vec<Scene>> {
        let mut scenes = Vec::new();
        let mut current_scene = Scene::new(None);
        
        self.advance(); // Skip the "Script" header
        
        while !self.is_at_end() {
            match self.current_token() {
                Token::LocationHeader(location) => {
                    if !current_scene.elements.is_empty() {
                        scenes.push(current_scene);
                    }
                    current_scene = Scene::new(Some(location.clone()));
                    self.advance();
                }
                Token::DialogueLine { speaker, text } => {
                    let element = ScriptElement::Dialogue {
                        speaker: speaker.clone(),
                        text: text.clone(),
                        actions: Vec::new(),
                    };
                    current_scene.elements.push(element);
                    self.advance();
                }
                Token::NarrationLine(text) => {
                    let element = ScriptElement::Narration(text.clone());
                    current_scene.elements.push(element);
                    self.advance();
                }
                Token::ActionText(text) => {
                    let element = ScriptElement::Action(text.clone());
                    current_scene.elements.push(element);
                    self.advance();
                }
                Token::SectionHeader(_) => break,
                _ => {
                    self.advance();
                }
            }
        }
        
        if !current_scene.elements.is_empty() {
            scenes.push(current_scene);
        }
        
        Ok(scenes)
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || matches!(self.current_token(), Token::EOF)
    }
}