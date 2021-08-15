use std::io;

// Terminal styling
use crossterm::{
    execute,
    style::{Color, ResetColor, SetColors, Colors, Print}
};

/// Provides an easy way to setup RGB colour codes to colourized output text
pub struct Style(pub u8, pub u8, pub u8);

pub fn stylized_output(style: &Style, display_text: &str) {
    let fg = Color::Rgb { 
        r: style.0,
        g: style.1,
        b: style.2
    };
    
    let bg = Color::Rgb{
        r: 0,
        g: 0,
        b: 0,
    };

    execute!(
        io::stdout(),
        SetColors(Colors::new(fg, bg)),
        Print(display_text),
        ResetColor
    ).expect("Failed to output stylized text.");
}
