extern crate termion;

use self::termion::event;


// List of event in this app
pub enum Event {
    Input(event::Key),
    Enter(String),
    Message(String),
    Quit,
    Error(String),
    Pseudo(String),
}

