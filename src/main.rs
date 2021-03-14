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

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    style::stylized_output(&intro, "Press p to pause, P to play, and e to exist.".to_string());
    println!();

    loop {
        let volume = sink.volume();

        key::detect(&sink, &mut stdout, &volume);
   }
}

fn main() {
    playback();
}