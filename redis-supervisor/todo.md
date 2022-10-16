# Redis Supervisor

## To Do List

* unit tests
* implement ping loop to ping instances based on settings in supervisor config
* eliminate conf files and pipe to redis-server start
* separate read_template into read and process to eliminate reading template multiple times
* create a Supervisor struct to hold config, template, state, etc
* move project to it's own repo (need name first rxkv?  redis-farm?)
* update apis to support ACL, JSON, etc
* design supervisor config file to specify instances
    * ~~redis.conf template file location (would override default)~~
    * ~~all template keys, i.e., port, password, max connections, etc~~
    * ~~auth source env, file, secrets?~~
    * any overrides from template (module at end of file)

###### dpw | 2022-10-16

