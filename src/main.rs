extern crate redis;
extern crate tui;
extern crate termion;

use redis::RedisResult;
use termion::raw::IntoRawMode;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time;
use std::io::{Write, stdout};
use std::thread;
use std::char;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, border, Widget, List, Paragraph};
use tui::layout::{Group, Rect, Direction, Size};
use tui::style::{Style, Color, Modifier};
use termion::event;
use termion::input::TermRead;

fn subscribe(channel: &str, client: redis::Client) -> redis::RedisResult<redis::PubSub> {
    let mut pubsub = try!(client.get_pubsub());
    try!(pubsub.subscribe(channel));

    Ok(pubsub)
}

fn ui(con: &redis::Connection) -> RedisResult<()> {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_text = String::new();
        io::stdin().read_line(&mut user_text)
            .expect("failed to read line");
    }
}

struct PubSubMessage{
    pubsub: redis::PubSub,
}

enum Event {
    Input(event::Key),
    Message(String),
    Refresh,
}

impl PubSubMessage{
    fn message_handler(&self, tx: &Sender<Event>) -> RedisResult<()> {
        loop {
            let msg = try!(self.pubsub.get_message());
            let payload : String = try!(msg.get_payload());
            tx.send(Event::Message(payload)).unwrap();
        }
    }
}

fn chat_start(tx: Sender<Event>, rx: Receiver<String>) -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://127.0.0.1:6379/"));
    let con = try!(client.get_connection());
    let pubsub = try!(subscribe("channel1", client));
    let message = PubSubMessage { pubsub : pubsub };

    thread::spawn(move || {
        message.message_handler(&tx);
    });

    thread::spawn(move || {
        loop {
            let user_text = rx.recv().unwrap();
            let _ : () = redis::cmd("PUBLISH").arg("channel1").arg(user_text).query(&con).unwrap();
        }
    });
    Ok(())
}

fn main() {
  let backend = TermionBackend::new().unwrap();
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.clear().unwrap();
  terminal.hide_cursor().unwrap();
  let (tx, rx) = mpsc::channel();
  let (tx_send_message, rx_message) = mpsc::channel();
  let input_tx = tx.clone();
  let message_tx = tx.clone();
  let clock_tx = tx.clone();
  let mut errors = vec![];


  thread::spawn(move || {
      let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
  });

 /* thread::spawn(move || {
      loop{
          message_tx.send(Event::Message).unwrap();
          thread::sleep(time::Duration::from_millis(500));
      }
  }); */

  thread::spawn(move || {
      loop{
          clock_tx.send(Event::Refresh).unwrap();
          thread::sleep(time::Duration::from_millis(400));
      }
  });
  chat_start(message_tx, rx_message);
  let mut isV = false;
  let mut buffer = String::new();
  loop {
    draw(&mut terminal, &errors, isV, &buffer);
    let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                if input == event::Key::Char('q') {
                    break;
                } 
                let i = match input {
                    event::Key::Char('q') => {
                        break;
                        "".to_string()
                    }, 
                    event::Key::Char(c) => { 
                        let mut a = char::to_string(&c);
                        if c == '\n' {
                            let ph = String::from(buffer.as_str());
                            tx_send_message.send(ph).unwrap();
                            buffer.clear();
                            a = "".to_string();
                        }
                        a
                    }
                    event::Key::Backspace => { 
                        buffer.pop(); 
                        "".to_string()
                        },
                    _ => "".to_string()
                };
                buffer = format!("{}{}", &buffer, &i);
            },
            Event::Refresh => {
                isV = !isV;
            },
            Event::Message(m) => {
                errors.insert(0, (m, "INFO"));
            }
        }
  }
  terminal.clear().unwrap();
  
  terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, errors: &Vec<(std::string::String, &str)>, isVisible: bool, buffer: &str) {
    let magenta = Style::default().fg(Color::Magenta);
  let red = Style::default().fg(Color::Red);
  let yellow = Style::default().fg(Color::Yellow);
  let white = Style::default().fg(Color::White);

  let size = t.size().unwrap();
  Group::default()
    .direction(Direction::Vertical)
    .margin(1)
    .sizes(&[Size::Percent(90), Size::Percent(10)])
    .render(t, &size, |t, chunks| {
        List::default()
                .block(Block::default()
                    .borders(border::ALL)
                    .title("Messages"))
                .items(&errors
                    .iter()
                    .map(|&(ref evt, level)| {
                        (format!("{}: {}", level, evt),
                         match level {
                            "ERROR" => &magenta,
                            "CRITICAL" => &red,
                            "WARNING" => &yellow,
                            _ => &white,
                        })
                    })
                    .collect::<Vec<(String, &Style)>>())
                .render(t, &chunks[0]);
        let squareChar = match isVisible {
            true => "▍",
            false => ""
        };
        let text = format!("> {}{}", buffer, &squareChar);
        Paragraph::default()
            .block(Block::default()
            .borders(border::ALL))
            .text(&text)
            .render(t, &chunks[1]);
    });
  
        t.draw().unwrap();
}