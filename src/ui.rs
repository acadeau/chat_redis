extern crate tui;
extern crate termion;

use self::tui::Terminal;
use self::tui::backend::TermionBackend;
use self::tui::widgets::{Block, border, Widget, List, Paragraph};
use self::tui::layout::{Group, Rect, Direction, Size};
use self::tui::style::{Style, Color};
use self::termion::event;
use self::termion::input::TermRead;
use event::Event;
use std::io;
use std::sync::mpsc::Sender;
use std::thread;


pub fn new(tx: Sender<Event>) -> (Terminal<TermionBackend>, Rect) {
        let backend = TermionBackend::new().unwrap();
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.hide_cursor().unwrap();
        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                let evt = c.unwrap();
                tx.send(Event::Input(evt)).unwrap();
            }
        });
        let size = terminal.size().unwrap();
        (terminal, size)
    }

pub fn draw(terminal: &mut Terminal<TermionBackend>, size: &Rect, buffer: &String, messages: &Vec<String>) {
        let white = Style::default().fg(Color::White);
        let text = format!("> {}{}", buffer, "▍");   
        let styled_messages = messages
                                .iter()
                                .map(|& ref message| {
                                    (message,
                                    & white)
                                })
                                .collect::<Vec<(&String, &Style)>>();
         
        Group::default()
            .direction(Direction::Vertical)
            .margin(1)
            .sizes(&[Size::Percent(90), Size::Percent(10)])
            .render( terminal
                   , size
                   , |t, chunks| {
                        List::default()
                            .block(Block::default()
                                    .borders(border::ALL)
                                    .title("Messages"))
                            .items(& styled_messages)
                            .render(t, &chunks[0]);

                        Paragraph::default()
                                .block(Block::default()
                                .borders(border::ALL))
                                .text(text.as_str())
                                .render(t, &chunks[1]);
                   });
        terminal.draw().unwrap();
    }

pub fn input_handler(event_tx: &Sender<Event>, key: event::Key, buffer: String) -> String {
        match key {
            event::Key::Char(c) =>  match c == '\n' {
                true => {
                    event_tx.send(Event::Enter(String::from(buffer.as_str()))).unwrap();
                    String::new()
                }
                false => format!("{}{}", buffer, c)
            },
            event::Key::Backspace => {
                let mut temp = String::from(buffer.as_str());
                temp.pop();
                temp
            },
            _ => String::from(buffer.as_str()),
        }
    }

pub fn clear(terminal: &mut Terminal<TermionBackend>){
    terminal.clear().unwrap();
    terminal.show_cursor().unwrap();
}

  