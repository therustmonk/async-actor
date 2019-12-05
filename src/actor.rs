use anyhow::Error;
use futures::channel::mpsc;
use futures::{select, SinkExt, StreamExt};
use std::net::Shutdown;
use tokio::net::TcpStream;

pub struct Actor {
    sender: mpsc::Sender<Msg>,
}

impl Actor {
    pub fn new(stream: TcpStream) -> Self {
        let (tx, rx) = mpsc::channel(16);
        let task = ActorTask {
            receiver: rx,
            stream,
        };
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
    stream: TcpStream,
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
                            self.stream.shutdown(Shutdown::Both)?;
                            break;
                        }
                    }
                }
                /* TODO: Wrap stream with codec and use here:
                data = self.stream.select_next_some() => {
                }
                */
            }
        }
        Ok(())
    }
}
