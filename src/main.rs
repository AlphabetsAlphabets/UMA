use std::io;
use std::io::stdout;
use std::fs::File;
use std::io::BufReader;

use rodio::Sink;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};

mod style;
mod key;

/*
TODO:
    1. Increase/decrease volume when J or up arrow, or when K or down arrow is pressed respectively
    2. Navigate the stream, forwards and backwards.
    3. Make output pretty, hide console logs.
*/
fn player_control(response: &String, sink: &Sink, mut control: &std::io::Stdout) -> bool {
    let response = response.trim();

    let positive = style::Style::new(
        [158, 252, 7],
    );

    let negative = style::Style::new(
        [234, 119, 121],
    );

    if response == "p" {
        sink.pause();
        execute!(control, Clear(ClearType::FromCursorUp)).unwrap();
        style::stylized_output(&positive, "Audio playback has beed paused.");
        println!();

        return false;
    }
    else if response == "e" {
        execute!(control, Clear(ClearType::FromCursorUp)).unwrap();
        return true; 
    }
    else {
        sink.play();
        execute!(control, Clear(ClearType::All)).unwrap();
        style::stylized_output(&negative, "Audio playback has resumed.");
        println!();
        return false;
    }
}

fn playback() {
    let mut stdout = stdout();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // opens the file, makes an audio source from the file
    let file = File::open("src/audio/audio.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Create a new thread to play audio on.
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    let intro = style::Style::new([252, 121, 7]);
    style::stylized_output(&intro, "Press p to pause, P to play, and e to exist.");

    execute!(stdout, Clear(ClearType::All)).unwrap();

    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .unwrap();

        let cancellation_token = player_control(&response, &sink, &mut stdout);
        if cancellation_token {
            let leave = style::Style::new(
                [200, 197, 247],
            );

            style::stylized_output(&leave, "Goodbye. See you next time.");
            break;
        }
        response.clear();
   }
}

fn main() {
    key::detect();
}