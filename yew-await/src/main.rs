use yew::prelude::*;

use crate::test::Test;

mod link;
mod test;
mod request;
mod global;

#[function_component]
fn App() -> Html {
  html! {
    <Test />
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
