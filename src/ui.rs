extern crate tui;
extern crate termion;

use self::tui::Terminal;
use self::tui::backend::TermionBackend;
use self::tui::widgets::{Block, border, Widget, List, Paragraph};
use self::tui::layout::{Group, Rect, Direction, Size};
use self::tui::style::{Style, Color, Modifier};
use self::termion::event;
use self::termion::input::TermRead;
use event::Event;
use std::io;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

struct Ui {
    terminal: Terminal<TermionBackend>,
    size: Rect,
    event_tx: Sender<Event>
}

impl Ui {
    fn new(&self, tx: Sender<Event>) -> Ui {
        let backend = TermionBackend::new().unwrap();
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();

        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                let evt = c.unwrap();
                self.event_tx.send(Event::Input(evt)).unwrap();
            }
        });

        Ui { terminal: terminal, size: terminal.size().unwrap(), event_tx: tx}
    }

    fn draw(& self, buffer: &String, messages: &Vec<String>) {
        terminal_screen()
            .render( &mut self.terminal
                   , &self.size
                   , |t, chunks| {
                       list_message(messages)
                         .render(t, &chunks[0]);
                       send_zone(buffer)
                         .render(t, &chunks[1]);
                   });
        self.terminal.draw().unwrap();
    }

    fn input_handler(&self, key: event::Key, buffer: &String) -> String {
        match key {
            event::Key::Char(c) =>  match c.eq('\n'.as_ref()) {
                true => {
                    self.event_tx.send(Event::Enter(String::from(buffer))).unwrap();
                    String::new()
                }
                false => c.to_string()
            },
            event::Key::Backspace => {
                let mut temp = String::from(&buffer);
                temp.pop();
                temp
            }
        }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        self.terminal.clear().unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

fn terminal_screen<'a>() ->  Group {
    Group::default()
    .direction(Direction::Vertical)
    .margin(1)
    .sizes(&[Size::Percent(90), Size::Percent(10)])
}

fn list_message <'a> (messages: &Vec<String>) -> List<'a> {
    List::default()
        .block(Block::default()
                .borders(border::ALL)
                .title("Messages"))
        .items(&messages)
}

fn send_zone(buffer: &String) -> Paragraph {

    let text = format!("> {}{}", buffer, "▍");    

    Paragraph::default()
            .block(Block::default()
            .borders(border::ALL))
            .text(text)
}


  