extern crate redis;

use event::Event;
use std::sync::mpsc::Sender;
use std::thread;
use self::redis::RedisResult;


// Struct for a chat, It has a redis connection
pub struct Chat {
    con: redis::Connection,
}

// Use to start the redis part of the app
// Return a chat struct
// It subscribes to a channel, and launch a thread to handle messages
pub fn start(tx: Sender<Event>) -> Chat {
        let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
        let con = client.get_connection().unwrap();

        let pubsub = subscribe("channel1", client).unwrap();

        thread::spawn(move || {
            message_handler(pubsub, &tx);
        });

        Chat { con: con }
}


impl Chat{
    // Send a message with Publish command
   pub fn send_message(&self, user_text: String) {
        let _ : () = redis::cmd("PUBLISH").arg("channel1").arg(user_text).query(&self.con).unwrap();
    }
}

// Function to handle message in the subscribed channel
fn message_handler(pubsub: redis::PubSub, tx: &Sender<Event>) -> RedisResult<()> {
        loop {
            let msg = try!(pubsub.get_message());
            let payload : String = try!(msg.get_payload());
            tx.send(Event::Message(payload)).unwrap();
        }
    }

// Subscribe to a channel pass in arguments
fn subscribe(channel: &str, client: redis::Client) -> redis::RedisResult<redis::PubSub> {
    let mut pubsub = try!(client.get_pubsub());
    try!(pubsub.subscribe(channel));

    Ok(pubsub)
}