use std::{sync::mpsc, thread};

fn main() {
  let  (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    tx.send(1).unwrap();
  });

  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
}
