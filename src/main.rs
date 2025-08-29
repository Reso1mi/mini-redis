use mini_redis::{Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Mini-Redis bind {addr} error!");

    println!("Mini-redis running on {addr}");

    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.expect("Mini-Redis Accept error!");
        println!("Conn Socket Addr: {socket_addr}");

        tokio::spawn(async {
            if let Err(e) = process(tcp_stream).await {
                println!("Connection failed: {}", e);
            }
        });
    }
}

async fn process(tcp_stream: TcpStream) -> Result<()> {
    let mut conn: Connection = Connection::new(tcp_stream);
    if let Some(frame) = conn.read_frame().await? {
        println!("Get Frame: {:?}", frame);
        let resp = Frame::Error("unimpl".into());
        conn.write_frame(&resp).await?;
    };

    Ok(())
}
