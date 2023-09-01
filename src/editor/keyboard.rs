use std::time::Duration;

use crossterm::event::{self, KeyEvent};

pub struct Keyboard {
}

impl Keyboard {
    // in the main loop of the program, read key event (non blocking with poll())
    pub fn read_key() -> Option<KeyEvent> {
        if event::poll(Duration::from_secs(0)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                return Some(key);
            }
        }
        None
    }
}