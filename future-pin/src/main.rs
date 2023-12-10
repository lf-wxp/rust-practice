// https://arc.net/l/quote/lgcendel

use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};

use futures::{Future, FutureExt};
use pin_project::pin_project;
use pin_utils::pin_mut;
use tokio::{
  fs::File,
  io::{AsyncRead, AsyncReadExt, ReadBuf},
  time::{sleep, Instant, Sleep},
};

struct MyFuture {
  sleep: Pin<Box<Sleep>>,
}

impl MyFuture {
  fn new() -> Self {
    Self {
      sleep: Box::pin(sleep(Duration::from_secs(1))),
    }
  }
}

impl Future for MyFuture {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("MyFuture::poll()");
    let mut sp = sleep(Duration::from_secs(1));
    pin_mut!(sp);
    let _ = sp.as_mut().poll(cx);
    self.sleep.as_mut().poll(cx)
  }
}
// #[tokio::main]
// async fn main() {
//   let fut = MyFuture::new();
//   println!("Awaiting fut...");
//   fut.await;
//   println!("Awaiting fut... done!");
// }

#[pin_project]
struct SlowRead<R> {
  #[pin]
  reader: R,
  #[pin]
  sleep: Sleep,
}

impl<R> SlowRead<R> {
  fn new(reader: R) -> Self {
    Self {
      reader,
      sleep: tokio::time::sleep(Default::default()),
    }
  }
}

impl<R> AsyncRead for SlowRead<R>
where
  R: AsyncRead + Unpin,
{
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<std::io::Result<()>> {
    //       👇            👇
    let mut this = self.project();

    match this.sleep.as_mut().poll(cx) {
      Poll::Ready(_) => {
        this.sleep.reset(Instant::now() + Duration::from_millis(25));
        this.reader.poll_read(cx, buf)
      }
      Poll::Pending => Poll::Pending,
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
  let mut buf = vec![0u8; 128 * 1024];
  let mut f = File::open("/dev/urandom").await?;
  let mut f = SlowRead::new(f);
  pin_utils::pin_mut!(f);
  let before = Instant::now();
  f.read_exact(&mut buf).await?;
  println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

  Ok(())
}
