use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

mod utils;
use utils::*;

mod proxy;
use proxy::Proxy;

// XXX replace hardcoded values with arguments from structopt

#[async_std::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let client = stream?;
        log(format!("New connection from: {}", client.peer_addr()?));
        let proxy = Proxy::new(client);
        task::spawn(async move {
            let _ = proxy.proxy_to("127.0.0.1:80".parse().unwrap()).await;
        });
    }

    Ok(())
}
