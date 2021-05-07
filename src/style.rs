use std::io;

// Terminal styling
use crossterm::{
    execute,
    style::{Color, ResetColor, SetColors, Colors, Print}
};

/// Provides an easy way to setup RGB colour codes to colourized output text
pub struct Style(pub u8, pub u8, pub u8);

/// ### Summary
/// Colourizes the text about to be printed onto the screen.alloc
/// 
/// ### Detailed explanation
/// With the `Style` struct to setup RGB color codes, `display_text` will have the colour
/// that belongs to the respective RGB code in the `Style` struct
/// 
/// ### Example
/// ```
/// let shade_of_green = Style::new([67, 178, 126]);
/// stylized_output(&shade_of_green, "This text is in a shade of green.".to_string());
/// ```
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
