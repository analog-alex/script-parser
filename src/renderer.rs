use crate::ast::{Script, ScriptElement};
use printpdf::*;
use anyhow::Result;
use std::fs::File;
use std::io::BufWriter;

pub struct PdfRenderer {
    font_size: f32,
    line_height: f32,
}

impl PdfRenderer {
    pub fn new() -> Self {
        PdfRenderer {
            font_size: 12.0,
            line_height: 14.0,
        }
    }
    
    pub fn render(&self, script: &Script, output_path: &str) -> Result<()> {
        let (doc, page1, layer1) = PdfDocument::new("Script", Mm(210.0), Mm(297.0), "Layer 1");
        let font = doc.add_builtin_font(BuiltinFont::Courier)?;
        
        let current_layer = doc.get_page(page1).get_layer(layer1);
        
        let mut y_position = 250.0;
        
        // Title section
        if !script.title_section.is_empty() {
            current_layer.use_text(&script.title_section, self.font_size, Mm(20.0), Mm(y_position), &font);
            y_position -= self.line_height * 2.0;
        }
        
        // Characters section
        if !script.characters.is_empty() {
            current_layer.use_text("CHARACTERS:", self.font_size, Mm(20.0), Mm(y_position), &font);
            y_position -= self.line_height;
            
            for (code, name) in &script.characters {
                let char_line = format!("{}: {}", code, name);
                current_layer.use_text(&char_line, self.font_size, Mm(25.0), Mm(y_position), &font);
                y_position -= self.line_height;
            }
            y_position -= self.line_height;
        }
        
        // Script content
        for scene in &script.scenes {
            if let Some(location) = &scene.location {
                let location_text = format!("[{}]", location);
                current_layer.use_text(&location_text, self.font_size, Mm(20.0), Mm(y_position), &font);
                y_position -= self.line_height * 1.5;
            }
            
            for element in &scene.elements {
                match element {
                    ScriptElement::Dialogue { speaker, text, .. } => {
                        let speaker_text = format!("{}:", speaker);
                        current_layer.use_text(&speaker_text, self.font_size, Mm(20.0), Mm(y_position), &font);
                        y_position -= self.line_height;
                        
                        current_layer.use_text(text, self.font_size, Mm(25.0), Mm(y_position), &font);
                        y_position -= self.line_height * 1.5;
                    }
                    ScriptElement::Narration(text) => {
                        current_layer.use_text(text, self.font_size, Mm(20.0), Mm(y_position), &font);
                        y_position -= self.line_height * 1.5;
                    }
                    ScriptElement::Action(text) => {
                        let action_text = format!("({})", text);
                        current_layer.use_text(&action_text, self.font_size, Mm(30.0), Mm(y_position), &font);
                        y_position -= self.line_height;
                    }
                }
                
                // Simple page break check
                if y_position < 30.0 {
                    break;
                }
            }
        }
        
        doc.save(&mut BufWriter::new(File::create(output_path)?))?;
        Ok(())
    }
}