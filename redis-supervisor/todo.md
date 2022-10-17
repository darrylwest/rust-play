# Redis Supervisor

## To Do List

* move project to it's own rxkv repo
* create a Supervisor struct to hold config, template, state, etc
* unit tests
* ~~replace tcp with unix sockets for primary connections~~ (in configuration)
* eliminate conf files and pipe to redis-server start
* separate read_template into read and process to eliminate reading template multiple times
* update apis to support ACL, JSON, etc
* ~~implement ping loop to ping instances based on settings in supervisor config~~
* ~~design supervisor config file to specify instances~~
    * ~~redis.conf template file location (would override default)~~
    * ~~all template keys, i.e., port, password, max connections, etc~~
    * ~~auth source env, file, secrets?~~
    * any overrides from template (module at end of file)

###### dpw | 2022-10-17

