use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};

use async_broadcast::{broadcast, Receiver};
use futures::{ready, Future, StreamExt};
use tokio::{
  spawn,
  time::sleep,
};

pub struct RequestFuture {
  id: String,
  receiver: Receiver<String>,
}

impl RequestFuture {
  pub fn new(id: String, receiver: Receiver<String>) -> Self {
    RequestFuture { id, receiver }
  }
}

impl Future for RequestFuture {
  type Output = String;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let this = self.get_mut();
    println!("poll before {}", &this.id);
    let msg = ready!(this.receiver.poll_next_unpin(cx));
    // let msg = ready!(Box::pin(this.receiver.recv()).poll_unpin(cx));
    println!("poll after {}", &this.id);
    if let Some(msg) = msg {
      if msg.contains(&this.id) {
        return Poll::Ready(msg);
      }
    }
    cx.waker().wake_by_ref();
    Poll::Pending
  }
}

#[tokio::main]
async fn main() {
  let (sender, receiver) = broadcast::<String>(10);
  let sender_clone = sender.clone();

  spawn(async move {
    sleep(Duration::from_secs(3)).await;
    let _ = sender_clone.broadcast_direct("message 1".to_string()).await;
    println!("send");
    sleep(Duration::from_secs(3)).await;
    let _ = sender_clone.broadcast_direct("message 2".to_string()).await;
  });

  let mut other = receiver.clone();
  spawn(async move {
    while let Ok(msg) = other.recv().await {
      println!("receive msg {:}", msg);
    }
  });

  let receiver = receiver.clone();
  let request_future = RequestFuture {
    id: "1".to_string(),
    receiver,
  };

  let result = request_future.await;
  println!("result: {}", result);
}
