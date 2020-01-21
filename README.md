# proxy-rs
Async TCP Proxy written in rust.

### Usage

```
proxy-rs 0.1.0
Generic TCP Proxy

To proxy all traffic on localhost port 80 to remote host 10.0.1.100 port 22 you would run:

proxy-rs -l 127.0.0.1:8080 -r 10.0.1.100:22

USAGE:
    proxy-rs --listen <listen> --remote <remote>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --listen <listen>    Specify the listening address i.e. 127.0.0.1:8080
    -r, --remote <remote>    Specify the remote address to proxy to i.e. 10.0.1.100:22
```

### Example

```
$ proxy-rs -l 127.0.0.1:8080 -r 10.0.1.156:22
2020-01-21 12:43:33 Proxy listening on: 127.0.0.1:8080
2020-01-21 12:43:36 New connection from: 127.0.0.1:44362
2020-01-21 12:43:42 Connection closed: (127.0.0.1:44362 -> 10.0.1.156:22) sent: 3.0 kB received: 14.1 kB
```
