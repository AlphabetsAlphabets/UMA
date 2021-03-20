use crossterm::event::{read, KeyEvent, KeyCode, Event, poll};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};
use rodio::Sink;

use std::time::Duration;

// #[path = "style.rs"] mod style;
use super::style;

fn on_key_detect(style: &style::Style, text: String, mut stdout: &std::io::Stdout) {
    //! ### Summary
    //! Provides colourized output to text in the terminal. 
    //! 
    //! ### Detailed explanation
    //! With `stdout` from `let stdout = std::io::stdout()`, the cursor will move to (0, 1) in the terminal
    //! and the screen will be cleared, then with the colourized text will be printed onto the screen.
    //! The colour is determined by what RGB values set in the `style` struct
    //! 
    //! ### Example
    //! ```
    //! let colourized = style::Style::new([252, 2, 202]);
    //! style::stylized_output(&colourized, "Colourized text!".to_string());
    //! ```
    execute!(stdout, cursor::MoveTo(0, 1), Clear(ClearType::FromCursorDown)).unwrap();
    style::stylized_output(&style, text);
    println!();
}

pub fn detect(sink: &Sink, stdout: &std::io::Stdout, volume: f32){
    //! ### Summray
    //! Used to detect keyboard inputs to issue commands such as pause, play, and exit.
    //! 
    //! ### Detailed explanation
    //! With the crossterm crate as a dependency, it will check whether or not there is keyboard input every
    //! second. If there is keyboard input then it'll continue, if not it will return from the funciton.
    let current_vol = style::Style::new([252, 2, 202]);

    // Checking if there is a keyboard input every second, if it doesn't, then it'll return the function.
    if !poll(Duration::from_secs(1)).unwrap_or_default() { return; }
    match read().unwrap() {
        Event::Key(KeyEvent { code: KeyCode::Char('j'), .. }) => {
            if volume == 0.0 { 
                let status = "Volume already at 0".to_string();
                on_key_detect(&current_vol, status, stdout);
                return;
            }
            let new_volume = volume - 0.25;
            sink.set_volume(new_volume);

            let new_volume = sink.volume().to_string();
            let status = format!("Current volume: {}", new_volume);
            on_key_detect(&current_vol, status, stdout);
        }

        Event::Key(KeyEvent { code: KeyCode::Char('k'), .. }) => {
            let new_volume = volume + 0.25;
            if volume == 4.0 { // Caps volume at 4, don't want it to burst my ear drums
                let status = "Volume is maxed out at 4.0".to_string();
                on_key_detect(&current_vol, status, stdout);
                return;
            }
            sink.set_volume(new_volume);

            let new_volume = sink.volume().to_string();
            let status = format!("Current volume: {}", new_volume);

            on_key_detect(&current_vol, status, stdout);
        }

        Event::Key(KeyEvent { code: KeyCode::Char('p'), .. }) => {
            let pause_play_style = style::Style::new([252, 105, 20]);
            if sink.is_paused() {
                on_key_detect(&pause_play_style, "Resuming audio playback.".to_string(), stdout);
                sink.play();
            }
            else {
                on_key_detect(&pause_play_style, "Halting audio playback.".to_string(), stdout);
                sink.pause();
            }
        }

        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
            sink.stop();
            let exit = style::Style::new([210, 118, 252]);
            style::stylized_output(&exit, "See you soon. Goodbye".to_string());
            println!();
            std::process::exit(200);
        }
        _ => ()
    }
}