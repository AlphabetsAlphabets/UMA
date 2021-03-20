use crossterm::event::{read, KeyEvent, KeyCode, Event};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};
use rodio::Sink;

// #[path = "style.rs"] mod style;
use super::style;

fn on_key_detect(style: &style::Style, text: String, mut stdout: &std::io::Stdout) {
    execute!(stdout, cursor::MoveTo(0, 1), Clear(ClearType::FromCursorDown)).unwrap();
    style::stylized_output(&style, text);
    println!();
}

pub fn detect(sink: &Sink, stdout: &std::io::Stdout, volume: f32){
    let current_vol = style::Style::new([252, 2, 202]);

    match read().unwrap() {
        Event::Key(KeyEvent { code: KeyCode::Char('j'), .. }) => {
            if volume == 0.0 { // Caps the lowest volume at 0 because for some reason volume increases when it goes to negative values
                let status = "Volume already at 0".to_string();
                on_key_detect(&current_vol, status, stdout);
                return;
            }
            let new_volume = volume - 0.5;
            sink.set_volume(new_volume);

            let new_volume = sink.volume().to_string();
            let status = format!("Current volume: {}", new_volume);
            on_key_detect(&current_vol, status, stdout);
        }

        Event::Key(KeyEvent { code: KeyCode::Char('k'), .. }) => {
            let new_volume = volume + 0.5;
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