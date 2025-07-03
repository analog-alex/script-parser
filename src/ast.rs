use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Script {
    pub title_section: String,
    pub characters: HashMap<String, String>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub location: Option<String>,
    pub elements: Vec<ScriptElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptElement {
    Dialogue { 
        speaker: String, 
        text: String, 
        actions: Vec<String> 
    },
    Narration(String),
    Action(String),
}

impl Script {
    pub fn new() -> Self {
        Script {
            title_section: String::new(),
            characters: HashMap::new(),
            scenes: Vec::new(),
        }
    }
}

impl Scene {
    pub fn new(location: Option<String>) -> Self {
        Scene {
            location,
            elements: Vec::new(),
        }
    }
}