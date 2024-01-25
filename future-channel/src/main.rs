use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};

use futures::{ready, Future, StreamExt};
use postage::{
  broadcast::{self, Receiver},
  sink::Sink,
};
use serde::{Deserialize, Serialize};
use tokio::{spawn, time::sleep};

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
  text: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum ResponseMessage {
  Media(Media),
  Log(String),
}

pub struct RequestMedia {
  receiver: Receiver<String>,
}

impl RequestMedia {
  pub fn new(receiver: Receiver<String>) -> Self {
    RequestMedia { receiver }
  }
}

impl Future for RequestMedia {
  type Output = Media;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let this = self.get_mut();
    let msg = ready!(this.receiver.poll_next_unpin(cx));
    if let Some(msg) = msg {
      match serde_json::from_str::<ResponseMessage>(&msg) {
        Ok(msg) => {
          println!("poll xxx {:?}", &msg);
          if let ResponseMessage::Media(message) = msg {
            println!("poll  after {:?}", &message);
            return Poll::Ready(message);
          }
          return Poll::Pending;
        }
        Err(_) => return Poll::Pending,
      }
    }
    Poll::Pending
  }
}

#[tokio::main]
async fn main() {
  let (mut sender, receiver) = broadcast::channel::<String>(10);
  let msg_handle = spawn(async move {
    println!("send start");
    sleep(Duration::from_secs(2)).await;
    let msg = serde_json::to_string(&ResponseMessage::Log("test".to_string())).unwrap();
    let _ = sender.blocking_send(msg);
    sleep(Duration::from_secs(2)).await;
    let msg = serde_json::to_string(&ResponseMessage::Media(Media {
      text: "test".to_string(),
    }))
    .unwrap();
    let _ = sender.blocking_send(msg);
    println!("send msg");
  });

  let mut other = receiver.clone();
  let other_receive = spawn(async move {
    while let Some(msg) = other.next().await {
      println!("receive msg {:}", msg);
    }
  });

  let await_handle = spawn(async move {
    println!("await start");
    let request_media = RequestMedia::new(receiver.clone());
    let msg = request_media.await;
    println!("await msg {:?}", msg);
  });

  await_handle.await.unwrap();
}
