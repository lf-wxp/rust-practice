use std::{cell::{Cell, RefCell}, rc::Rc};

fn main() {
  // code snipet 1
  let x = Cell::new(1);
  let y = &x;
  let z = &x;
  x.set(2);
  y.set(3);
  z.set(4);
  println!("{}", x.get());

  // code snipet 2
  let mut x = 1;
  let y = &mut x;
  let z = &mut x;
  x = 2;
  // *y = 3;
  // *z = 4;
  println!("{}", x);

  let s = Rc::new(RefCell::new(String::from("hello, world")));
  // let s1 = s.borrow();
  let s1 = s.clone();
  let mut s2 = s.borrow_mut();
  s2.push_str("xp");

  println!("{:?},{}",s1, s2);

}
