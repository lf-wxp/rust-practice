use std::time::Duration;

use futures::StreamExt;
use gloo_console::log;
use postage::{broadcast, sink::Sink};
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use crate::request::{Media, RequestMedia, ResponseMessage};

#[function_component]
pub fn Link() -> Html {
  use_effect_with((), move |_| {
    let (mut sender, mut receiver) = broadcast::channel::<String>(10);
    let (mut read_sender, read_receiver) = broadcast::channel::<String>(10);
    let (mut write_sender, mut write_receiver) = broadcast::channel::<String>(10);

    spawn_local(async move {
      while let Some(msg) = receiver.next().await {
        log!("broadcast msg receive", &msg);
        let _ = read_sender.blocking_send(msg.clone());
      }
    });
    spawn_local(async move {
      while let Some(msg) = write_receiver.next().await {
        log!("broadcast msg", &msg);
        let _ = sender.blocking_send(msg);
      }
    });

    log!("I got rendered, yay!");
    spawn_local(async move {
      log!("send start");
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Log("test".to_string())).unwrap();
      let _ = write_sender.blocking_send(msg);
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Media(Media {
        text: "test".to_string(),
      }))
      .unwrap();
      let _ = write_sender.blocking_send(msg.clone());
      log!("send msg", format!("{:?}", msg));
    });

    let mut other = read_receiver.clone();
    spawn_local(async move {
      while let Some(msg) = other.next().await {
        log!("receive msg {:}", msg);
      }
    });

    let rc = read_receiver.clone();
    spawn_local(async move {
      log!("await start");
      let request_media = RequestMedia::new(rc.clone());
      let msg = request_media.await;
      log!("await msg", format!("{:?}", msg));
    });
  });

  html! {
    <div>
      {{ "link" }}
    </div>
  }
}
