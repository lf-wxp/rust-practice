use std::sync::OnceLock;

use crate::link::Link;

static mut LINK: OnceLock<Link> = OnceLock::new();

pub fn get_link() -> Option<&'static mut Link> {
  unsafe {
    LINK.get_or_init(Link::new);
    LINK.get_mut()
  }
}
