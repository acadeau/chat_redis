# chat_redis

A chat app in terminal using `PUBLISH`/`SUBSCRIBE` of redis and Rust for learning purpose.

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
