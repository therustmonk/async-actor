use anyhow::Error;
use futures::channel::mpsc;
use futures::{select, SinkExt, StreamExt};

pub struct Actor {
    sender: mpsc::Sender<Msg>,
}

impl Actor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(16);
        let task = ActorTask { receiver: rx };
        tokio::spawn(task.entrypoint());
        Self { sender: tx }
    }

    async fn send_msg(&mut self, msg: Msg) -> Result<(), Error> {
        self.sender.send(msg).await.map_err(Error::from)
    }

    pub async fn terminate(&mut self) -> Result<(), Error> {
        self.send_msg(Msg::Terminate).await
    }
}

enum Msg {
    Terminate,
}

struct ActorTask {
    receiver: mpsc::Receiver<Msg>,
}

impl ActorTask {
    async fn entrypoint(mut self) {
        log::debug!("Actor started");
        if let Err(err) = self.routine().await {
            log::error!("Actor failed: {}", err);
        }
        log::debug!("Actor stopped");
    }

    async fn routine(&mut self) -> Result<(), Error> {
        loop {
            select! {
                msg = self.receiver.select_next_some() => {
                    match msg {
                        Msg::Terminate => {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
