tm
----

tm is a command line tool to handle a unix timestamp.

### Motivation

There is a number of times to generate/parse unix timestamps.
I think `date` command exists to handle these situations. But there are a few problems that they are small, but vital for me.
- cannot use same options between macOS and Linux.
- hard to remember usage. (it might be happen because of above problem.)

That's why I made a new command line tool `tm-cli`.

I hope tm-cli works well when developers need to use the command which requires timestamps like aws-cli.

### Example usage

Search logs from specific time period.
``` bash
# from yesterday to today
$ aws logs filter-log-events \
    --log-group-name <LOG_GROUP_NAME> \
    --log-stream-names <LOG_STREAM_NAMES> \
    --query <QUERY> \
    --start-time $(ut -p ms g -b yesterday) \
    --end-time $(ut -p ms g -b today)
```

### Installation

If you have rust toolchain, tm-cli can be installed with cargo.
``` bash
$ cargo install tm-cli
```

or clone the repository and build it.

``` bash
$ git clone https://github.com/yoshihitoh/tm-cli
$ cd tm-cli
$ cargo build --release
$ ./target/release/tm --version
tm 0.1.7
```

Also there are pre-built binary for Linux, macOS and Windows.
See [releases](https://github.com/yoshihitoh/tm-cli/releases).

### Usage
``` bash
tm-cli 0.2.1
yoshihitoh <yoshihito.arih@gmail.com>
A command line tool to handle unix timestamp.

USAGE:
    tm [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -u, --utc        Use utc timezone.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --offset <OFFSET>          Use given value as timezone offset.
    -p, --precision <PRECISION>
            Set the precision of output timestamp.


SUBCOMMANDS:
    generate    Generate unix timestamp with given options.
    help        Prints this message or the help of the given subcommand(s)
    parse       Parse a unix timestamp and print it in human readable format.
```

You can set options via envrionment variables.

| name               | equiv option   | example 
|:------------------:|:--------------:|:-----------
| UT_OFFSET          | -o/--offset    | 09:00
| UT_PRECISION       | -p/--precision | millisecond
| UT_DATETIME_FORMAT | -              | %Y-%m-%d %H:%M

UT_DATETIME_FORMAT follows chrono's datetime specifiers.
See [the document](https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html) for details.

```bash
# Set variables.
$ export UT_OFFSET='09:00'  # Use JST(+9).
$ export UT_PRECISION=millisecond  # Use timestamps in milliseconds.

# Generate a timestamp.
$ tm g
1588059756238

# Parse a timestamp.
$ echo 1588059756238 | tm p
2020-04-28 16:42:36.238 (+09:00)

# Change custom format and timezone.
$ export UT_DATETIME_FORMAT="%m/%d/%Y"
$ echo 1588059756238 | tm --offset=-7 p
04/28/2020
```

is equivalent to

```bash
$ tm -o '09:00' -p millisecond p $(tm -o '09:00' -p millisecond g)
```


There are two subcommands available for now.
- [generate(g)](#generate-a-unix-timestamp)
- [parse(p)](#parse-a-unix-timestamp)

#### Generate a unix timestamp

Generate a unix timestamp of the midnight of today.
``` bash
$ tm generate -b today
1560870000

# You can use `-p` option to show it in millisecond.
$ tm -p ms generate -b today
1560870000000
```

You can specify time deltas with `-d` option.
``` bash
# 3days, 12hours, 30minutes later from the midnight of today.
$ tm g -b today -d 3day -d 12hour -d 30minute
1561174200

# You can use short name on time unit.
$ tm g -b today -d 3d -d 12h -d 30min
1561174200

# You can modify a timestamp with a timestamp argument.
$ tm g -d 1min 1561174200
1561174260    # 1min(=60second) difference.
```

#### Parse a unix timestamp

Parse a unix timestamp and print it in human readable format.
``` bash
$ tm p $(tm g -b today)
2019-06-19 00:00:00 (+09:00)

# You can parse timestamp in milliseconds.
$ tm -p ms p $(tm -p ms g -b today -d 11h -d 22min -d 33s -d 444ms)
2019-06-19 11:22:33.444 (+09:00)
```

#### Change timezone

##### Local timezone
If you don't set timezone options, tm command uses local timezone.

In Japan(UTC+9):
``` bash
$ tm g --ymd 2019-06-24
1561302000

$ tm p 1561302000
2019-06-24 00:00:00 (+09:00)
```

You can use `-u` or `--utc` option to use UTC timezone.
``` bash
$ tm --utc p 1561302000
2019-06-23 15:00:00 (UTC)
```

You can use fixed offset timezone on any environment.
``` bash
# Generate PST timestamp
$ tm -o -8 g --ymd 2019-06-24
1561363200

# Parse as PST timestamp
$ tm -o -8 p 1561363200
2019-06-24 00:00:00 (-08:00)

# Parse as UTC timestamp
$ tm -o 0 p 1561363200
2019-06-24 08:00:00 (+00:00)
```

### TODO
- Add more information on README
