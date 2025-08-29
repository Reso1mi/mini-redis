use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "9999".into()).await?;

    let ret = client.get("hello").await?;

    println!("hello! = {:?}", ret);
    Ok(())
}
