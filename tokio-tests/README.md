# Tokio Stuff


## TCP Client / Server

* src/bin/server.r - a tcp server with mini-redis lib attached
* src/bin/client.rs - a tcp streamer to send messages to the server

### Server updates

* config for host / port
* logging
* create src/bin/tcp-server.rs - for the main() parts
* add other commands

### Client updates

* create src/bin/tcp-client.rs - for the main() parts
* read config
* add logging
* create repl to loop through commands

###### dpw | 2024.01.02

