# chat_redis

The purpose for this project use `SUBSCRIBE` and `PUBLISH` of redis command to have a chat.

I used Rust to learn this language.

## Dependencies :

* [Redis-rs](https://github.com/mitsuhiko/redis-rs) : Used to connect to redis 
* [Tui](https://github.com/fdehau/tui-rs) : Used for a ui in the terminal

## Command :

* `/q` or `/quit` : quit the application
* `/name arg` : change name to "arg"

## Known bugs :

There is a bug when we resize the terminal screen.

## Evolutions : 

* Better errors handling
