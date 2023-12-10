use std::{pin::{Pin, pin}, mem};

use pin_project::pin_project;

#[pin_project(!Unpin)]
#[derive(Debug)]
struct Struct<T, P> {
  field: T,
  #[pin]
  pin_field: P,
}

fn main() {
  let mut test = Struct { field: 5, pin_field: 1 };
  pin_utils::pin_mut!(test);
  // let mut pin = Pin::new(&mut test);
  let mut binding = test.as_mut().project();
  let mut a = binding.pin_field.as_mut();
  *a = 12;
  println!("after! {:?}", &test);
}
