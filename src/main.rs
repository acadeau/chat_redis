extern crate redis;
use redis::RedisResult;
use std::io;
use std::io::{Write};
use std::thread;

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
        let _ : () = try!(redis::cmd("PUBLISH").arg("channel1").arg(user_text).query(con));
    }
}

struct PubSubMessage {
    pubsub: redis::PubSub,
}

impl PubSubMessage {
    fn message_handler(&self) -> RedisResult<()> {
        loop {
            let msg = try!(self.pubsub.get_message());
            let payload : String = try!(msg.get_payload());
            println!("{}", payload);
        }
    }
}

fn chat_start() -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://127.0.0.1:6379/"));
    let con = try!(client.get_connection());
    let pubsub = try!(subscribe("channel1", client));
    let message = PubSubMessage { pubsub : pubsub };

    thread::spawn(move || {
        message.message_handler();
    });
    ui(&con);
    Ok(())
}


    
fn main() {
    println!("Hello, world!");
    chat_start();
}
