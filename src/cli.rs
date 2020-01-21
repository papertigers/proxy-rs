use async_std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Generic TCP Proxy
///
/// To proxy all traffic on localhost port 80 to remote host 10.0.1.100 port 22 you would run:
///
/// proxy-rs -l 127.0.0.1:8080 -r 10.0.1.100:22
pub struct Opt {
    #[structopt(name = "listen", long = "listen", short = "l")]
    /// Specify the listening address i.e. 127.0.0.1:8080
    pub listen: SocketAddr,
    #[structopt(name = "remote", long = "remote", short = "r")]
    /// Specify the remote address to proxy to i.e. 10.0.1.100:22
    pub remote: SocketAddr,
}

pub fn execute() -> Opt {
    Opt::from_args()
}
