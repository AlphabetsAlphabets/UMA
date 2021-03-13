use std::io;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;

fn player_control(mut response: String, sink: &Sink) {
    response = response.trim().to_string();
    if response == "p".to_string() {
            sink.pause();
            println!("Paused.");
            return;
        }
    else {
        sink.play();
        println!("Playing.");
    }
}

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = File::open("src/audio/audio.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    println!("Playing song.");

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    println!("p to pause, or play");

    loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .unwrap();

        let _ = player_control(response, &sink);
        println!("Status is paused: {}", sink.is_paused());
   }
}