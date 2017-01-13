extern crate redis;

use event::Event;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use self::redis::RedisResult;

struct Chat<'a> {
    con: &'a redis::Connection,
}

impl <'a> Chat<'a>{
    fn start(&self, tx: &Sender<Event>) -> Chat  {
        let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
        let con = client.get_connection().unwrap();

        let pubsub = subscribe("channel1", client).unwrap();

        thread::spawn(move || {
            message_handler(tx);
        });

        Chat { con: &con }
    }

    fn send_message(&self, user_text: String) {
        let _ : () = redis::cmd("PUBLISH").arg("channel1").arg(user_text).query(self.con).unwrap();
    }
    
}


fn message_handler(pubsub: redis::PubSub, tx: &Sender<Event>) -> RedisResult<()> {
        loop {
            let msg = try!(pubsub.get_message());
            let payload : String = try!(msg.get_payload());
            tx.send(Event::Message(payload)).unwrap();
        }
    }

fn subscribe(channel: &str, client: redis::Client) -> redis::RedisResult<redis::PubSub> {
    let mut pubsub = try!(client.get_pubsub());
    try!(pubsub.subscribe(channel));

    Ok(pubsub)
}