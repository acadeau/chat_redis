extern crate ui;
extern crate chat;
extern crate cmd;


struct App {
  messages: mut Vec<String>,
}

 

fn run(app: &App, ui: &Ui, chat: &Chatrx: &Receiver<Event>, tx_command: &Sender<Event>) {
    loop {
        let mut buffer = String::new();
        ui.draw(buffer, App.messages);
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                buffer = ui::input_handler(input, buffer);
            },
            Event::Enter(message) => match message.starts_with("/") {
                true => command::check_command(message, tx_command),
                false => chat.send_message(message),
            },
            Event::Message(message) => {
                App.messages.insert(0,message);
            }
            Event::Quit => break,
            Event::Error(err) =>  {
                App.messages.insert(0, err);
            }
        };
    }
}

fn main() {
  let app = { messages : [] };
  let (tx, rx) = mpsc::channel();
  let ui = Ui::new(tx.clone());
  Chat::start();
  run(&app, &ui, &rx, tx.clone());
}

