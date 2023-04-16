use async_std::task;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyRuntime;

impl MyRuntime {
  async fn run<F, T>(&self, future: F) -> T
  where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    task::spawn(future).await
  }
}

struct MyAsyncTask;

impl Future for MyAsyncTask {
  type Output = String;

  fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
    Poll::Ready("Hello, async world!".to_string())
  }
}

#[async_std::main]
async fn main() {
  let runtime = MyRuntime;
  let my_task = MyAsyncTask;
  let result = runtime.run(my_task).await;
  println!("Result: {}", result);
}
