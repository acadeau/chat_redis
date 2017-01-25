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
use std::cmp::min;


// Create a terminal for ui use.
// Launch a thread to handle user input
// @param {Sender<Event>} : A sender to communicate to the channel of the event.
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


// Draw the ui
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
                        // List widget to display messages
                        List::default()
                            .block(Block::default()
                            .borders(border::ALL)
                            .title("Messages"))
                            .items(& styled_messages)
                            .render(t, &chunks[0]);
                        
                        // Paragraph widget to show the user entry
                        Paragraph::default()
                                .block(Block::default()
                                .borders(border::ALL))
                                .text(text.as_str())
                                .render(t, &chunks[1]);
                   });
        terminal.draw().unwrap();
    }

// Manage the user input. Handle the enter key and the backspace key
pub fn input_handler(event_tx: &Sender<Event>, key: event::Key, buffer: String) -> String {
        match key {
            event::Key::Char(c) =>  match c == '\n' && !buffer.is_empty()   {
                true => {
                    event_tx.send(Event::Enter(String::from(buffer.as_str()))).unwrap();
                    String::new()
                }
                false => String::from(format!("{}{}", buffer, c).replace("\n", ""))
            },
            event::Key::Backspace => {
                let mut temp = String::from(buffer.as_str());
                temp.pop();
                temp
            },
            _ => String::from(buffer.as_str()),
        }
    }

// Clear the ui
pub fn clear(terminal: &mut Terminal<TermionBackend>){
    terminal.clear().unwrap();
    terminal.show_cursor().unwrap();
}

// Use to have the maximum of elements in the list widget
pub fn get_nb_elements_per_list(size: &Rect, messages: &Vec<String>) -> usize{
    // Don't know why this is 73%, maybe the size of character
    ((size.height * 73) / 100) as usize
}