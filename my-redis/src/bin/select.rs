use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
  let (mut tx1, mut rx1) = mpsc::channel(128);
  let (mut tx2, mut rx2) = mpsc::channel(128);

  tokio::spawn(async move {
    // 用 tx1 和 tx2 干一些不为人知的事
    tx2.send("tx2");
    tx1.send("tx1").await;
  });

  tokio::select! {
    Some(v) = rx1.recv() => {
      println!("Got {:?} from rx1", v);
    }
    Some(v) = rx2.recv() => {
      println!("Got {:?} from rx2", v);
    }
    else => {
      println!("Both channels closed");
    }
  };

  let (tx3, mut rx3) = mpsc::channel(128);
  let (tx4, mut rx4) = mpsc::channel(128);
  let (tx5, mut rx5) = mpsc::channel(128);

  tokio::spawn(async move {
    tx3.send("tx3").await;
    tx4.send("tx4").await;
    tx5.send("tx5").await;
  });

  loop {
    let msg = tokio::select! {
      Some(msg) = rx3.recv() => msg,
      Some(msg) = rx4.recv() => msg,
      Some(msg) = rx5.recv() => msg,
        else => { break }
    };

    println!("Got {:?}", msg);
  }

  println!("All channels have been closed.");

}
