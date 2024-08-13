// Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let shared_reply_reference = Arc::new(RwLock::new(reply));
    let handle_first = tokio::spawn(single_reply(first, shared_reply_reference.clone()));
    let handle_second = tokio::spawn(single_reply(second, shared_reply_reference.clone()));

    let (outcome1, outcome2) = tokio::join!(handle_first, handle_second);
    let _ = outcome1.unwrap();
    let _ = outcome2.unwrap();
}

pub async fn single_reply<T>(listener: TcpListener, reply_reference: Arc<RwLock<T>>)
where
    T: Display + Send + Sync + 'static,
{
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let handle_reference = reply_reference.clone(); // needed to move into the following spawn

        tokio::spawn(async move {
            let (mut _reader, mut writer) = socket.split();
            let cloned_reference;
            {
                // the lock lives only in this scope, but cloned_reference outlives the lock
                cloned_reference = handle_reference.read().unwrap().to_string().clone();
            }
            writer.write_all(cloned_reference.as_bytes()).await.unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
