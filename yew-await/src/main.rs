use yew::prelude::*;

use crate::link::Link;

mod link;
mod request;

#[function_component]
fn App() -> Html {
  html! {
    <Link />
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
