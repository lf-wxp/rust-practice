use std::{
  pin::Pin,
  task::{Context, Poll},
};

use futures::{ready, Future, StreamExt};
use postage::broadcast::Receiver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
  pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseMessage {
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
