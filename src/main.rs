use std::fs::File;
use std::io::stdout;
use std::io::BufReader;

use crossterm::terminal::{disable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use rodio::Sink;

use std::path::PathBuf;

mod config;
mod files;
mod key;
mod style;

/// ### Summary
/// Plays the select audio file via the provided file path
///
/// ### Detailed explanation
/// It creates an audio source from the result of `File::open(file_to_play)`, appends it to the sink
/// and plays the audio file. It can also detect keyboard inputs while audio is playing, this is used to
/// control the volume, and to exit the player.
///
/// ### An example
/// ```
/// let path_to_audio_file = <path to audio file>.to_string();
/// playback(&path_to_audio_file);
/// ```
fn playback(file_to_play: &[PathBuf]) {
    let mut stdout = stdout();
    let (_stream, stream_handle) =
        { rodio::OutputStream::try_default().expect("Unable to create output stream from file.") };

    let file_path = file_to_play[0].as_path();

    // opens the file, makes an audio source from the file
    let file = File::open(file_path).expect("Unable to open file.");
    let source = rodio::Decoder::new(BufReader::new(file));
    let source = match source {
        Ok(source) => source,
        Err(_error) => {
            let error_style = style::Style(244, 25, 9);
            style::stylized_output(
                &error_style,
                "The file you've chosen has an unrecognized format.",
            );
            std::process::abort()
        }
    };

    // Create a new thread to play audio on.
    let sink = Sink::try_new(&stream_handle).expect("unable to create sink.");
    sink.append(source);

    loop {
        let volume = sink.volume();

        key::detect(&sink, &mut stdout, volume);
        if sink.empty() {
            break;
        }
    }
    disable_raw_mode().expect("Unable to disable raw mode.");
    execute!(stdout, cursor::Show).expect("unable to show cursor");
}

fn play(file_path: &Vec<PathBuf>) {
    let mut stdout = stdout();

    let file_name = files::get_song_names(&file_path[..]);
    let song_to_play = files::select_song(&file_name[..], &file_path[..]);

    let current_song = files::get_song_names(&song_to_play[..]);
    let current_song_style = style::Style(135, 244, 9);

    // Convert: PathBuff -> OsStr -> OsString -> String, then match on Result
    let current_song = {
        current_song[0]
            .as_os_str()
            .to_owned()
            .into_string()
            .expect("Unable to convert to String.")
    };

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown)
    )
    .expect("Unable to execute crossterm functions");
    let current_song = format!("Currently playing: {}", current_song);
    style::stylized_output(&current_song_style, &current_song);
    println!();

    playback(&song_to_play[..]);

    loop {
        let file_name = files::get_song_names(&file_path);
        let song_to_play = files::select_song(&file_name, &file_path);

        // If you pass in a vec with only one element, it doesn't pass the value in...
        let current_song = files::get_song_names(&song_to_play[..]);
        let current_song = {
            current_song[0]
                .as_os_str()
                .to_owned()
                .into_string()
                .expect("cannot convert to String.")
        };

        let current_song = format!("Currently playing: {}", current_song);

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::FromCursorDown)
        )
        .expect("unable to execute crossterm functions");

        style::stylized_output(&current_song_style, &current_song);
        println!();

        playback(&song_to_play);
    }
}

fn main() {
    let conf = config::modify_json();

    let file_path = match files::get_file_extension(conf.dir) {
        Some(file_path) => file_path,
        None => std::process::abort(),
    };

    if conf.first == 0 {
        play(&file_path);
    }
    play(&file_path);
}
