use std::{
  pin::Pin,
  task::{Context, Poll},
};

use futures::{ready, Future, FutureExt};
use serde::{Deserialize, Serialize};
use tokio::{spawn, sync::broadcast::{self, Receiver, Sender} };

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
  pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
  pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Data {
  Media(Media),
  Log(Log),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
  id: String,
  data: Data,
}

pub struct Request {
  sender: Sender<String>,
  receiver: Receiver<String>,
}

impl Request {
  pub fn new(sender: Sender<String>, receiver: Receiver<String>) -> Self {
    Request { receiver, sender }
  }

  pub fn request(&mut self, data: Data, id: String) -> RequestFuture {
    println!("id {:?}", id.clone());
    let message = serde_json::to_string(&ResponseMessage {
      data,
      id: id.clone(),
    })
    .unwrap();
    let sender = self.sender.clone();
    let receiver = self.sender.subscribe();
    let future = RequestFuture::new(id, receiver);
    let _ = sender.send(message);
    future
  }
}

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
  type Output = Data;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let this = self.get_mut();
    println!("poll before {}", &this.id);
    let msg = ready!(Box::pin(this.receiver.recv()).poll_unpin(cx));
    if let Ok(msg) = msg {
      match serde_json::from_str::<ResponseMessage>(&msg) {
        Ok(ResponseMessage { data, id }) => {
          println!("poll xxx {:?}, {:?}, {:?}", &this.id, &id, &msg);
          if id == this.id {
            println!("poll  after {:?}", &msg);
            return Poll::Ready(data);
          }
          println!("poll pending xxx {:?}, {:?}, {:?}", &this.id, &id, &msg);
          return Poll::Pending;
        }
        Err(_) => return Poll::Pending,
      }
    }
    return Poll::Pending;
  }
}

#[tokio::main]
async fn main() {
  let (sender, receiver) = broadcast::channel::<String>(10);

  let mut other = sender.subscribe();
  let other_receive = spawn(async move {
    while let Ok(msg) = other.recv().await {
      println!("receive msg {:}", msg);
    }
  });
  let receiver_clone = sender.subscribe();
  let sender_clone = sender.clone();
  let await_handle1 = spawn(async move {
    println!("await start1");
    let mut request = Request::new(sender_clone, receiver_clone);
    let msg = request
      .request(
        Data::Media(Media {
          text: "media".to_string(),
        }),
        "1".to_string(),
      )
      .await;
    println!("await msg1 {:?}", &msg);
  });

  let receiver_clone = sender.subscribe();
  let sender_clone = sender.clone();
  let await_handle2 = spawn(async move {
    println!("await start2");
    let mut request = Request::new(sender_clone, receiver_clone);
    let msg = request
      .request(
        Data::Log(Log {
          text: "log".to_string(),
        }),
        "2".to_string(),
      )
      .await;
    println!("await msg2, {:?} ", &msg);
  });

  await_handle2.await.unwrap();
}
