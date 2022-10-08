use std::collections::HashMap;

use mini_redis::{Command::{self, Get, Set}, Connection, Frame};
use tokio::net::TcpStream;

pub async fn process(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };

        conn.write_frame(&resp).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_server() {
        let srv_socket = TcpListener::bind("localhost:8888").await.unwrap();

        loop {
            let (cli_socket, _) = srv_socket.accept().await.unwrap();
            tokio::spawn(async move {
                process(cli_socket).await;
            });
        }
    }
}
