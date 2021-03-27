use std::io;

// Terminal styling
use crossterm::{
    execute,
    style::{Color, ResetColor, SetColors, Colors, Print}
};

pub struct Style {
    /// Provides an easy way to setup RGB colour codes to colourized output text
    fg: [u8; 3],
}

impl Style {
    /// Provides an easy way to create an instance of Style
    pub fn new(fg: [u8; 3]) -> Style {
        Style {
            fg,
        }
    }
}

pub fn stylized_output(style: &Style, display_text: &str) {
    //! ### Summary
    //! Colourizes the text about to be printed onto the screen.alloc
    //! 
    //! ### Detailed explanation
    //! With the `Style` struct to setup RGB color codes, `display_text` will have the colour
    //! that belongs to the respective RGB code in the `Style` struct
    //! 
    //! ### Example
    //! ```
    //! let shade_of_green = Style::new([67, 178, 126]);
    //! stylized_output(&shade_of_green, "This text is in a shade of green.".to_string());
    //! ```
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