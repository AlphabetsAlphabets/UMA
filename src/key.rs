use crossterm::event::{read, KeyEvent, KeyCode, Event, KeyModifiers};

pub fn detect() {
    loop {
        let key_detect = style::Style::new([25, 104, 252]);
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE
            }) => style::stylized_output(&key_detect, "Up arrow."),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE
            }) => style::stylized_output(&key_detect, "Down arrow."),
            _ => (),
        }
    }
}