use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

mod cli;
mod utils;
use utils::*;

mod proxy;
use proxy::Proxy;

#[async_std::main]
async fn main() -> io::Result<()> {
    let opt = cli::execute();
    let remote = opt.remote;
    let listener = TcpListener::bind(opt.listen).await?;
    log(format!("Proxy listening on: {}", opt.listen));
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let client = stream?;
        log(format!("New connection from: {}", client.peer_addr()?));
        let proxy = Proxy::new(client);
        task::spawn(async move {
            match proxy.proxy_to(remote).await {
                Ok(_) => (),
                Err(e) => log(format!("[Error] {}", e)),
            }
        });
    }

    Ok(())
}
