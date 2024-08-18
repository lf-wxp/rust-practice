use async_broadcast::{broadcast, Receiver, Sender};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug)]
pub struct Link {
  sender: Sender<String>,
  pub receiver: Receiver<String>,
  read_sender: Sender<String>,
}

impl Link {
  pub fn new() -> Self {
    let (write, mut read) = broadcast::<String>(10);
    let (write_sender, write_receiver) = broadcast::<String>(20);
    let (read_sender, read_receiver) = broadcast::<String>(20);
    let sender_clone = read_sender.clone();
    spawn_local(async move {
      while let Ok(msg) = read.recv().await {
        let _ = sender_clone.broadcast_direct(msg.clone()).await;
      }
    });
    let mut receiver_clone = write_receiver.clone();
    spawn_local(async move {
      while let Ok(msg) = receiver_clone.recv().await {
        let _ = write.broadcast_direct(msg).await;
      }
    });
    Link {
      sender: write_sender,
      receiver: read_receiver,
      read_sender,
    }
  }

  pub fn sender(&self) -> Sender<String> {
    self.sender.clone()
  }
  pub fn receiver(&self) -> Receiver<String> {
    self.read_sender.new_receiver()
  }
}
