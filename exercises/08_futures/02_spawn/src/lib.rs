use std::panic;

use tokio::{net::TcpListener, task::JoinSet};

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    // top-level orchestration:
    // we want 2 independent listener loops running concurrently

    let handle1 = tokio::spawn(async move {
        // listener task 1

        // infinite listener loop:
        // keep accepting new clients forever
        loop {
            // wait for next client on first listener
            let (mut stream, _) = first.accept().await.unwrap();

            // accepting connections is one layer of waiting.
            // each individual client connection should ALSO run concurrently,
            // otherwise one slow client would block future accepts.
            //
            // so we spawn a separate task per client connection.
            tokio::spawn(async move {
                let (mut reader, mut writer) = stream.split();

                // continuously copy bytes from client back to client
                // until the client closes the connection
                tokio::io::copy(&mut reader, &mut writer)
                    .await
                    .unwrap();
            });
        }
    });

    let handle2 = tokio::spawn(async move {
        // listener task 2

        // same logic as first listener, but for the second socket
        loop {
            let (mut stream, _) = second.accept().await.unwrap();

            tokio::spawn(async move {
                let (mut reader, mut writer) = stream.split();

                tokio::io::copy(&mut reader, &mut writer)
                    .await
                    .unwrap();
            });
        }
    });

    // wait for both listener tasks concurrently.
    //
    // in practice these never finish because both listener loops are infinite,
    // so this keeps the server alive forever.
    tokio::try_join!(handle1, handle2)?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
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
