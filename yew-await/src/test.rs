use std::time::Duration;
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use crate::{global::get_link, request::{Media, RequestMedia, ResponseMessage}};

#[function_component]
pub fn Test() -> Html {
  use_effect_with((), move |_| {
    let link = get_link().unwrap(); 
    let mut other = link.receiver();

    spawn_local(async move {
      while let Ok(msg) = other.recv().await {
        log!("receive msg {:}", msg);
      }
    });

    let rc = link.receiver();
    spawn_local(async move {
      log!("await start");
      let request_media = RequestMedia::new(rc);
      let msg = request_media.await;
      log!("await msg", format!("{:?}", msg));
    });

    let sender = link.sender();
    spawn_local(async move {
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Log("test".to_string())).unwrap();
      let _ = sender.broadcast_direct(msg.clone()).await;
      log!("send msg", format!("{:?}", msg));
      sleep(Duration::from_secs(2)).await;
      let msg = serde_json::to_string(&ResponseMessage::Media(Media {
        text: "test".to_string(),
      }))
      .unwrap();
      let _ = sender.broadcast_direct(msg.clone()).await;
      log!("send msg", format!("{:?}", msg));
    });
  });

  html! {
    <div>
      {{ "link" }}
    </div>
  }
}
