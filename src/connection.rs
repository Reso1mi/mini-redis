use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // 分配一个缓冲区，具有4kb的缓冲长度
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        // 具体实现
        self.parse
    }

    /// 将帧写入到连接中
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        // 具体实现
    }
}
