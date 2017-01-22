use event::Event;
use std::sync::mpsc::Sender;

pub fn check_command (cmd: String, tx: &Sender<Event>) {
  match cmd.as_str() {
    "/quit" => {
      tx.send(Event::Quit).unwrap();
    },
    "/q" => {
      tx.send(Event::Quit).unwrap();
    },
    _ => {
      tx.send(Event::Error("Command not found".to_string())).unwrap();
    },
  };
}