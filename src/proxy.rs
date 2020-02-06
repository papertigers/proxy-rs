use crate::utils::*;
use async_std::io;
use async_std::io::{Read, Write};
use async_std::net::{SocketAddr, TcpStream};
use async_std::prelude::*;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub struct Proxy {
    stream: TcpStream,
    received_bytes: AtomicU64,
    sent_bytes: AtomicU64,
    start: Instant,
}

impl Proxy {
    /// Construct a new `Proxy` for a given `TcpStream`
    pub fn new(stream: TcpStream) -> Self {
        Proxy {
            stream,
            received_bytes: AtomicU64::new(0),
            sent_bytes: AtomicU64::new(0),
            start: Instant::now(),
        }
    }

    /// Connect the `Proxy` to a remote address and spawn tasks that shuffle data in each direction
    pub async fn proxy_to(&self, remote: SocketAddr) -> io::Result<()> {
        let remote = TcpStream::connect(remote).await?;
        let conn_str = format!("({} -> {})", self.stream.peer_addr()?, remote.peer_addr()?);

        // Setup the TcpStreams that are going to be proxied to each other.
        let (client_rx, client_tx) = &mut (&self.stream, &self.stream);
        let (remote_rx, remote_tx) = &mut (&remote, &remote);
        let send = self.pipe(client_rx, remote_tx, true);
        let recv = self.pipe(remote_rx, client_tx, false);

        // Run both pipe futures in parallel and stop if either of them fails.
        match futures::try_join!(send, recv) {
            Ok(_) => log(format!("Connection closed: {} {}", conn_str, self.stats())),
            Err(e) => log(format!("[Error] {}: {} {}", e, conn_str, self.stats())),
        }
        Ok(())
    }

    /// Proxy the src into the dst and record how much data is sent in the specefied direction.
    /// The only reason we use this over `io::copy()` is that we want to keep track of some proxy
    /// statistics.
    async fn pipe<R, W>(&self, src: &mut R, dst: &mut W, sending: bool) -> io::Result<()>
    where
        R: Read + Unpin + ?Sized,
        W: Write + Unpin + ?Sized,
    {
        // Note: this could probably be improved to mimic io::copy() but this impl is probably good
        // enough for now.
        let mut buf = [0; 0xffff]; // 64K buffer
        loop {
            let nbytes = src.read(&mut buf).await?;
            // We reached EOF
            if nbytes == 0 {
                return Ok(());
            }
            match dst.write_all(&buf[..nbytes]).await {
                Ok(_) => {
                    if sending {
                        self.sent_bytes.fetch_add(nbytes as u64, Ordering::SeqCst);
                    } else {
                        self.received_bytes
                            .fetch_add(nbytes as u64, Ordering::SeqCst);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Return prettified string of the `Proxy`'s stats
    fn stats(&self) -> String {
        format!(
            "sent: {} received: {} ({})",
            pretty_bytes(self.sent_bytes.load(Ordering::SeqCst)),
            pretty_bytes(self.received_bytes.load(Ordering::SeqCst)),
            duration_delta(self.start)
        )
    }
}
