use std::collections::HashMap;

use mini_redis::cmd::{Get, Set};
use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Tiny-Redis bind {addr} error!");

    println!("Tiny-redis running on {addr}");

    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.expect("Tiny-Redis Accept error!");
        println!("Conn Socket Addr: {socket_addr}");

        tokio::spawn(async {
            if let Err(e) = process(tcp_stream).await {
                println!("Connection failed: {}", e);
            }
        });
    }
}

async fn process(tcp_stream: TcpStream) -> Result<()> {
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();

    let mut conn: Connection = Connection::new(tcp_stream);
    while let Some(frame) = conn.read_frame().await? {
        let response = match Command::from_frame(frame)? {
            Command::Get(get) => {
                if let Some(val) = db.get(get.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            Command::Set(set) => {
                db.insert(set.key().to_string(), set.value().to_vec());
                Frame::Simple("OK".into())
            }
            _ => Frame::Error("Unimplment".into()),
        };

        // 响应客户端
        conn.write_frame(&response).await?;
    }

    if let Some(frame) = conn.read_frame().await? {
        println!("Get Frame: {:?}", frame);
        let resp = Frame::Error("unimpl".into());
        conn.write_frame(&resp).await?;
    };

    Ok(())
}
