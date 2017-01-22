extern crate tui;
extern crate termion;

mod chat;
mod ui;
mod event;
mod cmd;

use self::tui::Terminal;
use self::tui::backend::TermionBackend;
use self::tui::layout::Rect;
use chat::Chat;
use std::sync::mpsc::{Sender, Receiver};
use event::Event;
use std::sync::mpsc;


 

fn run(terminal: &mut Terminal<TermionBackend>, size: &Rect, chat: &Chat, rx: &Receiver<Event>, tx: Sender<Event>) {
    let tx_ui = tx.clone();
    let tx_command = tx.clone();
    let mut messages = Vec::new();
    let mut buffer = String::new();
    
    loop {
        ui::draw(terminal, size, &buffer, &messages);
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                buffer = ui::input_handler(&tx_ui, input, buffer);
            },
            Event::Enter(message) => match message.starts_with("/") {
                true => cmd::check_command(message, &tx_command),
                false => chat.send_message(message),
            },
            Event::Message(message) => {
                messages.insert(0,message);
            }
            Event::Quit => break,
            Event::Error(err) =>  {
                messages.insert(0, err);
            }
        };
    }
}

fn main() {
  let (tx, rx) = mpsc::channel();
  let (mut terminal, size) = ui::new(tx.clone());
  let chat = chat::start(tx.clone());
  run(&mut terminal, &size, &chat, &rx, tx.clone());
  ui::clear(&mut terminal)
}

