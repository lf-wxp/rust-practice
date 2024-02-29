use std::time::Duration;

use futures::StreamExt;
use gloo_console::log;
use async_broadcast::broadcast;
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use crate::request::{Media, RequestMedia, ResponseMessage};

#[function_component]
pub fn Link() -> Html {
  use_effect_with((), move |_| {
    let (sender, mut receiver) = broadcast::<String>(10);
    let (read_sender, read_receiver) = broadcast::<String>(10);
    let (write_sender, mut write_receiver) = broadcast::<String>(10);

    spawn_local(async move {
      while let Ok(msg) = receiver.recv().await {
        log!("broadcast msg receive", &msg);
        let _ = read_sender.broadcast_direct(msg.clone()).await;
      }
    });
    spawn_local(async move {
      while let Ok(msg) = write_receiver.recv().await {
        log!("broadcast msg", &msg);
        let _ = sender.broadcast_direct(msg).await;
      }
    });

    log!("I got rendered, yay!");
    spawn_local(async move {
      log!("send start");
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Log("test".to_string())).unwrap();
      let _ = write_sender.broadcast_direct(msg).await;
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Media(Media {
        text: "test".to_string(),
      }))
      .unwrap();
      let _ = write_sender.broadcast_direct(msg.clone()).await;
      log!("send msg", format!("{:?}", msg));
    });

    let mut other = read_receiver.clone();
    spawn_local(async move {
      while let Ok(msg) = other.recv().await {
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
