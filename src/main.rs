extern crate redis;
use redis::RedisResult;

fn do_something() -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://127.0.0.1:6379/"));
    let con = try!(client.get_connection());
    let mut pubsub = try!(client.get_pubsub());

    try!(pubsub.subscribe("channel1"));

    let _ : RedisResult<()> = redis::cmd("PUBLISH").arg("channel1").arg("test").query(&con);
    println!("ok");
    loop {
        let msg = try!(pubsub.get_message());
        let payload : String = try!(msg.get_payload());
        println!("{}", payload);
    }
    /* do something here */
}

fn main() {
    println!("Hello, world!");

    do_something();
    

}
