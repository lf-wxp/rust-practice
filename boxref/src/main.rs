use std::{ops::Deref, fmt::Display};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // add code here
  fn new(val: T) -> MyBox<T> {
    MyBox(val)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: Display> Display for MyBox<T> {
    // add code here
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:}", self.0)
  }
}

fn main() {
  let mybox = MyBox::new(5);
  let y = *mybox + 1;
  println!("the my box is {:}", mybox);
  println!("the y is {:}", y);
}
