mod actor;

use actor::Actor;
use anyhow::Error;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::delay_for;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let mut actor = Actor::new(stream);
        tokio::spawn(async move {
            delay_for(Duration::from_secs(5)).await;
            actor.terminate().await.ok();
        });
    }
}
