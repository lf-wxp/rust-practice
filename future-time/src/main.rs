
mod executor;
mod timer;

use std::time::Duration;

use crate::executor::new_executor_and_spawner;

use crate::timer::TimerFuture;
// ANCHOR: main
fn main() {
  let (executor, spawner) = new_executor_and_spawner();

  // Spawn a task to print before and after waiting on a timer.
  spawner.spawn(async {
    println!("howdy!");
    // Wait for our timer future to complete after two seconds.
    TimerFuture::new(Duration::new(10, 0)).await;
    println!("done!");
  });

  // Drop the spawner so that our executor knows it is finished and won't
  // receive more incoming tasks to run.
  drop(spawner);

  // Run the executor until the task queue is empty.
  // This will print "howdy!", pause, and then print "done!".
  executor.run();
}
// ANCHOR_END: main
