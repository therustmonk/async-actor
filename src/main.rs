mod actor;

use actor::Actor;
use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let mut actor = Actor::new();
    actor.terminate().await?;
    Ok(())
}
