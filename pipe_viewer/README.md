# Pipe Viewer

This code originated from the training video [Hands On System Programming with Rust](https://www.udemy.com/course/hands-on-systems-programming-with-rust/) and has been enhanced to add:

* configuration struct as parsed from cli
* macro based cli parsing
* added logging
* unit and integrtion tests

The application reads from a file or stdin and writes to stdout or a specified file while gathering stats on the piped stream.  Read, Write and stats are gathered in three threads that communicate via channels.


## To Do

* logging
* improved error handling
* unit and integration tests


###### darryl.west | 2022-09-07

