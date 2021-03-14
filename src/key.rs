use crossterm::event::{read, KeyEvent, KeyCode, Event, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};
use rodio::Sink;

// #[path = "style.rs"] mod style;
use super::style;

pub fn detect(sink: &Sink, mut control: &std::io::Stdout, volume: &f32){
    loop {
        let current_vol = style::Style::new([252, 2, 202]);

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE
            }) => {
                let new_volume = volume - 0.5;
                sink.set_volume(new_volume);

                let new_volume = sink.volume().to_string();
                let status = format!("Current volume: {}", new_volume);

                execute!(control, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                style::stylized_output(&current_vol, status);

                println!();
                return;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE
            }) => {
                let new_volume = volume + 0.5;
                sink.set_volume(new_volume);

                let new_volume = sink.volume().to_string();
                let status = format!("Current volume: {}", new_volume);

                execute!(control, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                style::stylized_output(&current_vol, status);

                println!();
                return;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE
            }) => {
                let pause_play_style = style::Style::new([252, 105, 20]);
                if sink.is_paused() {
                    sink.play();
                    style::stylized_output(&pause_play_style, "Resuming audio playback.".to_string());

                    println!();
                    return;
                }
                sink.pause();
                style::stylized_output(&pause_play_style, "Halting audio playback.".to_string());

                println!();
                return;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE
            }) => {
                sink.stop();
                let exit = style::Style::new([210, 118, 252]);
                style::stylized_output(&exit, "See you soon. Goodbye".to_string());
                println!();
                std::process::exit(200);
            }
            _ => ()
        }
    }
}