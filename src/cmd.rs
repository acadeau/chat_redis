use event::Event;
use std::sync::mpsc::Sender;

pub fn check_command (cmd: String, tx: &Sender<Event>) {
  let mut cmd_split = cmd.split_whitespace();
  
  match cmd_split.next().unwrap() {
    "/quit" => {
      tx.send(Event::Quit).unwrap();
    },
    "/q" => {
      tx.send(Event::Quit).unwrap();
    },
    "/name" => {
      match cmd_split.next() {
        Some(arg) => tx.send(Event::Pseudo(arg.to_string())).unwrap(),
        None => tx.send(Event::Pseudo("Error : Missing arguments : /name arg1".to_string())).unwrap(),
      }
    },
    _ => {
      tx.send(Event::Error("Error : Command not found".to_string())).unwrap();
    },
  };
}