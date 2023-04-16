use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
  let a = Rc::new(String::from("rc-value"));
  let b = Rc::clone(&a);

  println!("the rc value is {}", a);
  println!("the rc clone value is {}", b);
  println!("the rc  count value is {}", Rc::strong_count(&a));

  let s = Arc::new(String::from("arc-value"));

  for _ in 0..10 {
    let s = Arc::clone(&s);
    let _handle = thread::spawn(move || println!("{}", s));
  }
}
