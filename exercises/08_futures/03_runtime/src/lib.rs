// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    // T itself is not cloneable
    // We need to share it across
    // - listener task 1
    // - listener task 2
    // - every client connection task
    // Arc gives shared ownership without cloning underlying T
    // Arc clones only clones the shared ownership pointer, not the underlying T

    let reply = Arc::new(reply); // can be owned by listener task 1
    let reply_another = reply.clone(); // can be owned by listener task 2

    // listener task 1
    let handle1 = tokio::spawn(async move {
        loop {
            let (mut stream, _) = first.accept().await.unwrap();

            // we cannot move 'reply' into the client task. We need to clone it.
            // If not, we will lose the ownership of arc pointer to be used by subsequent client tasks
            let reply_client_clone = reply.clone();
            tokio::spawn(async move {
                let (_, mut writer) = stream.split();
                let response = reply_client_clone.to_string();
                writer.write_all(response.as_bytes()).await.unwrap();
            });
        }
    });

    // listener task 2
    let handle2 = tokio::spawn(async move {
        loop {
            let (mut stream, _) = second.accept().await.unwrap();

            let reply_another_client_clone = reply_another.clone();
            tokio::spawn(async move {
                let (_, mut writer) = stream.split();
                let response = reply_another_client_clone.to_string();
                writer.write_all(response.as_bytes()).await.unwrap();
            });
        }
    });

    let _ = tokio::join!(handle1, handle2);
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
