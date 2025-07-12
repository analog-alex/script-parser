use crate::ast::{Script, ScriptElement};
use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug)]
pub struct ValidationError {
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        ValidationError {
            line: None,
            column: None,
            message,
            suggestion: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

pub struct Validator {
    errors: Vec<ValidationError>,
    warnings: Vec<ValidationError>,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn validate(&mut self, script: &Script) -> Result<()> {
        self.errors.clear();
        self.warnings.clear();

        // Validate script structure
        self.validate_script_structure(script)?;
        
        // Validate character definitions
        self.validate_characters(script)?;
        
        // Validate script content
        self.validate_script_content(script)?;
        
        // Validate reserved keywords
        self.validate_reserved_keywords(script)?;
        
        // Validate nesting and formatting
        self.validate_formatting(script)?;

        if !self.errors.is_empty() {
            let error_messages: Vec<String> = self.errors
                .iter()
                .map(|e| self.format_error(e))
                .collect();
            return Err(anyhow!("Validation failed:\n{}", error_messages.join("\n")));
        }

        if !self.warnings.is_empty() {
            let warning_messages: Vec<String> = self.warnings
                .iter()
                .map(|e| self.format_warning(e))
                .collect();
            eprintln!("Warnings:\n{}", warning_messages.join("\n"));
        }

        Ok(())
    }

    fn validate_script_structure(&mut self, script: &Script) -> Result<()> {
        // Check if title section exists
        if script.title_section.is_empty() {
            self.errors.push(
                ValidationError::new("Title section is missing".to_string())
                    .with_suggestion("Add a title section with '# Title' header".to_string())
            );
        }

        // Check if characters section exists
        if script.characters.is_empty() {
            self.errors.push(
                ValidationError::new("Character definitions are missing".to_string())
                    .with_suggestion("Add character definitions in the '## Characters' section".to_string())
            );
        }

        // Check if script content exists
        if script.scenes.is_empty() {
            self.errors.push(
                ValidationError::new("Script content is missing".to_string())
                    .with_suggestion("Add script content in the '## Script' section".to_string())
            );
        }

        Ok(())
    }

    fn validate_characters(&mut self, script: &Script) -> Result<()> {
        let mut character_codes: HashSet<String> = HashSet::new();

        // Collect all character codes from definitions
        for (code, _) in &script.characters {
            character_codes.insert(code.clone());
        }

        // Check for conflicts with reserved keywords
        if script.characters.contains_key("N") {
            self.warnings.push(
                ValidationError::new("Character code 'N' is reserved for narrator".to_string())
                    .with_suggestion("Consider using a different code for this character".to_string())
            );
        }

        // Validate character code format (should be uppercase letters)
        for (code, name) in &script.characters {
            if !code.chars().all(|c| c.is_ascii_uppercase()) {
                self.errors.push(
                    ValidationError::new(format!("Invalid character code '{}': must contain only uppercase letters", code))
                        .with_suggestion("Use only uppercase letters for character codes".to_string())
                );
            }

            if code.is_empty() {
                self.errors.push(
                    ValidationError::new("Character code cannot be empty".to_string())
                        .with_suggestion("Provide a valid character code".to_string())
                );
            }

            if name.trim().is_empty() {
                self.errors.push(
                    ValidationError::new(format!("Character name for code '{}' cannot be empty", code))
                        .with_suggestion("Provide a valid character name".to_string())
                );
            }
        }

        // Check for duplicate character codes
        let mut seen_codes = std::collections::HashSet::new();
        for (code, _) in &script.characters {
            if !seen_codes.insert(code) {
                self.errors.push(
                    ValidationError::new(format!("Duplicate character code '{}'", code))
                        .with_suggestion("Use unique codes for each character".to_string())
                );
            }
        }

        Ok(())
    }

    fn validate_script_content(&mut self, script: &Script) -> Result<()> {
        let mut used_characters: HashSet<String> = HashSet::new();
        let character_codes: HashSet<String> = script.characters.keys().cloned().collect();

        for (scene_index, scene) in script.scenes.iter().enumerate() {
            for (_element_index, element) in scene.elements.iter().enumerate() {
                match element {
                    ScriptElement::Dialogue { speaker, text, actions } => {
                        // Check if speaker is defined
                        if !character_codes.contains(speaker) && speaker != "N" {
                            self.errors.push(
                                ValidationError::new(format!("Undefined character code '{}' used in dialogue", speaker))
                                    .with_suggestion(format!("Add '{}: Character Name' to the character definitions", speaker))
                            );
                        }

                        used_characters.insert(speaker.clone());

                        // Validate dialogue text
                        if text.trim().is_empty() {
                            self.errors.push(
                                ValidationError::new(format!("Empty dialogue for character '{}'", speaker))
                                    .with_suggestion("Provide dialogue text or remove the line".to_string())
                            );
                        }

                        // Validate actions within dialogue
                        for action in actions {
                            if action.trim().is_empty() {
                                self.errors.push(
                                    ValidationError::new("Empty action description".to_string())
                                        .with_suggestion("Provide action text or remove the action".to_string())
                                );
                            }
                        }
                    }
                    ScriptElement::Narration(text) => {
                        if text.trim().is_empty() {
                            self.errors.push(
                                ValidationError::new("Empty narration text".to_string())
                                    .with_suggestion("Provide narration text or remove the line".to_string())
                            );
                        }
                    }
                    ScriptElement::Action(text) => {
                        if text.trim().is_empty() {
                            self.errors.push(
                                ValidationError::new("Empty action text".to_string())
                                    .with_suggestion("Provide action text or remove the line".to_string())
                            );
                        }
                    }
                }
            }

            // Validate scene structure
            if scene.elements.is_empty() {
                self.warnings.push(
                    ValidationError::new(format!("Scene {} has no content", scene_index + 1))
                        .with_suggestion("Add dialogue, narration, or action to the scene".to_string())
                );
            }
        }

        // Check for unused character definitions
        for (code, name) in &script.characters {
            if !used_characters.contains(code) {
                self.warnings.push(
                    ValidationError::new(format!("Character '{}' ({}) is defined but never used", name, code))
                        .with_suggestion("Remove unused character or add dialogue for this character".to_string())
                );
            }
        }

        Ok(())
    }

    fn validate_reserved_keywords(&mut self, script: &Script) -> Result<()> {
        // Check for conflicts with reserved keywords
        let reserved_keywords = vec!["N"];

        for keyword in reserved_keywords {
            if script.characters.contains_key(keyword) {
                self.warnings.push(
                    ValidationError::new(format!("Character code '{}' is reserved for narrator", keyword))
                        .with_suggestion("Consider using a different code for this character".to_string())
                );
            }
        }

        Ok(())
    }

    fn validate_formatting(&mut self, script: &Script) -> Result<()> {
        // Validate scene structure
        for (scene_index, scene) in script.scenes.iter().enumerate() {
            // Check for consecutive empty scenes
            if scene.elements.is_empty() {
                self.warnings.push(
                    ValidationError::new(format!("Scene {} has no content", scene_index + 1))
                        .with_suggestion("Add content to the scene or remove it".to_string())
                );
            }

            // Check for proper scene transitions
            if let Some(location) = &scene.location {
                if location.trim().is_empty() {
                    self.errors.push(
                        ValidationError::new("Scene location cannot be empty".to_string())
                            .with_suggestion("Provide a valid location name".to_string())
                    );
                }
            }
        }

        Ok(())
    }

    fn format_error(&self, error: &ValidationError) -> String {
        let mut formatted = format!("ERROR: {}", error.message);
        
        if let (Some(line), Some(column)) = (error.line, error.column) {
            formatted = format!("{} (line {}, column {})", formatted, line, column);
        }
        
        if let Some(suggestion) = &error.suggestion {
            formatted = format!("{}\n  Suggestion: {}", formatted, suggestion);
        }
        
        formatted
    }

    fn format_warning(&self, warning: &ValidationError) -> String {
        let mut formatted = format!("WARNING: {}", warning.message);
        
        if let (Some(line), Some(column)) = (warning.line, warning.column) {
            formatted = format!("{} (line {}, column {})", formatted, line, column);
        }
        
        if let Some(suggestion) = &warning.suggestion {
            formatted = format!("{}\n  Suggestion: {}", formatted, suggestion);
        }
        
        formatted
    }
} 