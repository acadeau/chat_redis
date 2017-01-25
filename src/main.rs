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


// Run the main loop, It will handle all events.
fn run(terminal: &mut Terminal<TermionBackend>, size: &Rect, chat: &Chat, rx: &Receiver<Event>, tx: Sender<Event>) {
    let tx_ui = tx.clone();
    let tx_command = tx.clone();
    let mut messages = Vec::new();
    let mut buffer = String::new();
    let mut pseudo = String::from("anonymous");
    loop {
        let nb_elements = ui::get_nb_elements_per_list(size, &messages);
        ui::draw(terminal, size, &buffer, &messages);
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                buffer = ui::input_handler(&tx_ui, input, buffer);
            },
            Event::Enter(message) => match message.starts_with("/") {
                true => cmd::check_command(message, &tx_command),
                false => chat.send_message(format!("{} : {}", pseudo, message)),
            },
            Event::Message(message) => {
                if nb_elements == messages.len() && nb_elements != 0 {
                    messages.remove(0);
                }
                messages.push(message);
            }
            Event::Quit => break,
            Event::Error(err) =>  {
                if nb_elements == messages.len() && nb_elements != 0 {
                    messages.remove(0);
                }
                messages.push(err);
            },
            Event::Pseudo(new_pseudo) => {
                chat.send_message(format!("Info : {} is now {}.", pseudo, new_pseudo));
                pseudo = new_pseudo;
            }
        };
    }
}

fn main() {
  // Creation of a channel to communicate with the main loop.
  let (tx, rx) = mpsc::channel();
  let (mut terminal, size) = ui::new(tx.clone());
  let chat = chat::start(tx.clone());
  run(&mut terminal, &size, &chat, &rx, tx.clone());
  ui::clear(&mut terminal)
}

