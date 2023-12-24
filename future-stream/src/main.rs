use std::{
  cell::RefCell,
  ops::Range,
  pin::Pin,
  rc::Rc,
  task::{Poll, Waker}, sync::{Mutex, Arc}, borrow::BorrowMut,
};

use futures::{channel::mpsc, ready, task::SpawnExt, Sink, SinkExt, Stream, StreamExt};
use tokio::spawn;

pub type Error = Box<dyn std::error::Error>;

struct Pipe {
  value: Range<i32>,
  waker:Arc<Mutex<Option<Waker>>>,
  message_receiver: mpsc::UnboundedReceiver<i32>,
  message_sender: mpsc::UnboundedSender<i32>,
  is_ready: bool,
}

impl Pipe {
  fn new() -> Pipe {
    let value = 0..100;
    let (sender, receiver) = mpsc::unbounded();
    Pipe {
      value,
      waker: Arc::new(Mutex::new(None)),
      message_receiver: receiver,
      message_sender: sender,
      is_ready: false,
    }
  }
  fn set_ready(&mut self, val: bool) {
    self.is_ready = val;
  }

  fn setup(&self) {
    let mut waker = self.waker.lock().unwrap();
    if let Some(waker) = waker.take() {
      waker.wake();
    }
  }
}

impl Sink<i32> for Pipe {
  type Error = Error;

  fn poll_ready(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    if self.is_ready {
      Poll::Ready(Ok(()))
    } else {
      let mut waker = self.waker.lock().unwrap();
      *waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }

  fn start_send(mut self: Pin<&mut Self>, item: i32) -> Result<(), Self::Error> {
    println!("start_send");
    let _ = self.message_sender.send(item);
    Ok(())
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    println!("poll_flush");
    Poll::Ready(Ok(()))
  }

  fn poll_close(
    self: Pin<&mut Self>,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    println!("poll_close");
    Poll::Ready(Ok(()))
  }
}

impl Stream for Pipe {
  type Item = i32;

  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let msg = ready!(self.message_receiver.poll_next_unpin(cx));
    Poll::Ready(msg)
  }
}

#[tokio::main]
async fn main() {
  let mut pipe = Pipe::new();
  pipe.setup();
  pipe.set_ready(true);
  let (mut write, mut read) = pipe.split();
  let manager = spawn(async move {
    while let Some(val) = read.next().await {
      println!("read value is {}", val);
    }
  });
  println!("Hello, world!");
  let _ = manager.await;
}
