use std::{
  pin::Pin,
  task::{Context, Poll},
};

use async_broadcast::Receiver;
use futures::{ready, Future, StreamExt};
use gloo_console::log;
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
    log!("poll xxx ", format!("{:?}", &msg));
    if let Some(msg) = msg {
      match serde_json::from_str::<ResponseMessage>(&msg) {
        Ok(msg) => {
          if let ResponseMessage::Media(message) = msg {
            return Poll::Ready(message);
          }
          log!("not media msg");
        }
        Err(_) => return Poll::Pending,
      }
    }
    // 如果未解析到有效的Media消息，唤醒当前任务以便在下一次事件循环时继续尝试接收消息。
    cx.waker().wake_by_ref();
    log!("before pending");
    Poll::Pending
  }
}
