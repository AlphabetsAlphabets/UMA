use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use rodio::Sink;

use std::time::Duration;

use super::config;
use super::style;

fn on_key_detect(style: &style::Style, text: &str, mut stdout: &std::io::Stdout) {
    execute!(
        stdout,
        cursor::MoveTo(0, 1),
        Clear(ClearType::FromCursorDown)
    )
    .expect("Unable to execute.");
    style::stylized_output(&style, text);
    println!();
}

pub fn detect(sink: &Sink, mut stdout: &std::io::Stdout, volume: f32) {
    let current_vol = style::Style(252, 2, 202);

    enable_raw_mode().expect("unable to enable raw mode.");
    execute!(stdout, cursor::Hide, cursor::MoveTo(0, 1)).expect("unable to execute.");
    // Checking if there is a keyboard input every second, if it doesn't, then it'll return.
    if !poll(Duration::from_secs(1)).unwrap_or_default() {
        return;
    }

    match read().expect("Unable to read keyboard inputs.") {
        Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            ..
        }) => {
            if volume == 0.0 {
                let status = "Volume already at 0";
                on_key_detect(&current_vol, status, stdout);
                return;
            }
            let new_volume = volume - 0.05;
            sink.set_volume(new_volume);

            let new_volume = sink.volume().to_string();
            let status = format!("Current volume: {}", new_volume);
            on_key_detect(&current_vol, &status, stdout);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            ..
        }) => {
            let new_volume = volume + 0.05;
            if volume == 2.0 {
                // Caps volume at 4, don't want it to burst my ear drums
                let status = "Volume is maxed out at 2.0";
                on_key_detect(&current_vol, status, stdout);
                return;
            }
            sink.set_volume(new_volume);

            let new_volume = sink.volume().to_string();
            let status = format!("Current volume: {}", new_volume);

            on_key_detect(&current_vol, &status, stdout);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            ..
        }) => {
            let pause_play_style = style::Style(252, 105, 20);
            if sink.is_paused() {
                on_key_detect(&pause_play_style, "Resuming audio playback.", stdout);
                sink.play();
            } else {
                on_key_detect(&pause_play_style, "Halting audio playback.", stdout);
                sink.pause();
            }
        }

        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => {
            sink.stop();
            let exit = style::Style(210, 118, 252);
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))
                .expect("Unable to execute.");
            style::stylized_output(&exit, "See you soon. Goodbye");
            println!();

            execute!(stdout, cursor::Show).expect("Unable to show cursor");
            std::process::exit(200);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('C'),
            modifiers: KeyModifiers::SHIFT,
        }) => {
            sink.stop();
            config::reset_json();

            execute!(stdout, cursor::Show).expect("Unable to execute.");

            let reset_style = style::Style(255, 255, 255);
            let reset_message = "Go ahead and restart uma.";
            style::stylized_output(&reset_style, reset_message);

            std::process::abort();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            ..
        }) => {
            sink.stop();
        }
        _ => (),
    }
}
