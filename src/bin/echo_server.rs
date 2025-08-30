use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // 较为重量级的拆分，底层会使用Arc和Mutex
            // let (mut reader, mut writer) = io::split(socket);
            // 获取字节流的引用，分离为写入器和读取器，底层不会使用锁但是使用了引用，reader和writer脱离当前作用域后不可用
            let (mut reader, mut writer) = socket.split();
            if io::copy(&mut reader, &mut writer).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
// nc 127.0.0.1 6666
