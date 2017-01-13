extern crate termion;

use self::termion::event;

pub enum Event {
    Input(event::Key),
    Enter(String),
    Message(String),
    Quit,
    Error(String),
}

