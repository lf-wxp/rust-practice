use std::{
  pin::Pin,
  sync::{Arc, Mutex},
  task::{Context, Poll, Waker},
  time::Duration,
};

use futures::{channel::mpsc, Sink, SinkExt, Stream, StreamExt};
use tokio::{spawn, time::sleep};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

struct Pipe {
  waker: Arc<Mutex<Option<Waker>>>,
  message_receiver: mpsc::UnboundedReceiver<i32>,
  message_sender: mpsc::UnboundedSender<i32>,
  ready: Arc<Mutex<bool>>,
  flushed: Arc<Mutex<bool>>,
}

impl Pipe {
  fn new() -> Pipe {
    let (sender, receiver) = mpsc::unbounded();
    Pipe {
      waker: Arc::new(Mutex::new(None)),
      message_receiver: receiver,
      message_sender: sender,
      ready: Arc::new(Mutex::new(false)),
      flushed: Arc::new(Mutex::new(false)),
    }
  }
}

impl Sink<i32> for Pipe {
  type Error = Error;

  fn start_send(self: Pin<&mut Self>, item: i32) -> Result<(), Self::Error> {
    let this = self.get_mut();
    println!("start_send");
    this
      .message_sender
      .unbounded_send(item)
      .map_err(|e| Box::new(e) as Error)
  }

  fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    let this = self.get_mut();
    let ready = this.ready.lock().unwrap();
    println!("poll_ready {}", *ready);
    if *ready {
      Poll::Ready(Ok(()))
    } else {
      let mut waker = this.waker.lock().unwrap();
      *waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    let this = self.get_mut();
    let flushed = this.flushed.lock().unwrap();
    println!("poll_flush {}", *flushed);
    if *flushed {
      Poll::Ready(Ok(()))
    } else {
      let mut waker = this.waker.lock().unwrap();
      *waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }

  fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }
}

impl Stream for Pipe {
  type Item = Result<i32, Error>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let this = self.get_mut();
    println!("poll_next");
    match Pin::new(&mut this.message_receiver).poll_next(cx) {
      Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
      Poll::Ready(None) => Poll::Ready(None),
      Poll::Pending => {
        let mut waker = this.waker.lock().unwrap();
        *waker = Some(cx.waker().clone());
        Poll::Pending
      }
    }
  }
}

#[tokio::main]
async fn main() {
  let pipe = Pipe::new();
  let waker = pipe.waker.clone();
  let waker_clone = pipe.waker.clone();
  let ready = pipe.ready.clone();
  let flushed = pipe.flushed.clone();
  let (mut write, mut read) = pipe.split();
  let read_handle = spawn(async move {
    println!("read");
    while let Some(val) = read.next().await {
      println!("read value is {}", val.unwrap());
    }
  });
  let write_handle = spawn(async move {
    println!("write");
    let _ = write.send(22).await;
  });
  let ready_handle = spawn(async move {
    sleep(Duration::from_secs(2)).await;
    println!("ready");
    *ready.lock().unwrap() = true;
    if let Some(waker) = waker.lock().unwrap().as_ref() {
      waker.wake_by_ref();
    }
  });
  let flushed_handle = spawn(async move {
    sleep(Duration::from_secs(5)).await;
    println!("flush");
    *flushed.lock().unwrap() = true;
    if let Some(waker) = waker_clone.lock().unwrap().as_ref() {
      waker.wake_by_ref();
    }
  });
  write_handle.await.unwrap();
  read_handle.await.unwrap();
  // ready_handle.await.unwrap();
  // flushed_handle.await.unwrap();
  println!("Hello, world!");
}
