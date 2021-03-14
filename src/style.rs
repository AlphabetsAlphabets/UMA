use std::io;

// Terminal styling
use crossterm::{
    execute,
    style::{Color, ResetColor, SetColors, Colors, Print}
};

// Easy way to setup stylized output 
pub struct Style {
    fg: [u8; 3],
}

impl Style {
    pub fn new(fg: [u8; 3]) -> Style {
        Style {
            fg,
        }
    }
}

pub fn stylized_output(style: &Style, display_text: &str) {
    let fg = Color::Rgb { 
        r: style.fg[0],
        g: style.fg[1],
        b: style.fg[2],
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