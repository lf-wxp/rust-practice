use std::error::Error;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{
  mpsc::{self, Sender},
  oneshot,
};
type BoxError = Box<dyn Error + Send + Sync>;

type Responder<T> = oneshot::Sender<T>;
type CmdBaseResult<T> = Result<T, BoxError>;

#[derive(Debug)]
enum CmdResult {
  val(CmdBaseResult<Option<Bytes>>),
  empty(CmdBaseResult<()>),
}

#[derive(Debug)]
enum Command {
  Get {
    key: String,
    resp: Option<Responder<CmdResult>>,
  },
  Set {
    key: String,
    val: Bytes,
    resp: Option<Responder<CmdResult>>,
  },
}

impl Command {
  fn get_resp(old: Option<Responder<CmdResult>>, new: Responder<CmdResult>) -> Option<Responder<CmdResult>> {
    if old.is_none() {
      return Some(new);
    }
    old
  }

  fn attach_resp(self, sender: Responder<CmdResult>) -> Command {
    match self {
      Command::Get { key, resp } => {
        let resp = Command::get_resp(resp, sender);
        Command::Get { key, resp }
      }
      Command::Set { key, val, resp } => {
        let resp = Command::get_resp(resp, sender);
        Command::Set { key, val, resp }
      }
    }
  }
}

async fn task(cmd: Command, tx: Sender<Command>) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel::<CmdResult>();
    // Send the GET request
    let cmd = cmd.attach_resp(resp_tx);
    if tx.send(cmd).await.is_err() {
      eprintln!("connection task shutdown");
      return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Get) = {:?}", res.unwrap());
  })
}

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);
  // Clone a `tx` handle for the second f
  let tx2 = tx.clone();

  let manager = tokio::spawn(async move {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(cmd) = rx.recv().await {
      match cmd {
        Command::Get { key, resp } => {
          let res = client.get(&key).await;
          // Ignore errors
          let _ = resp.unwrap().send(CmdResult::val(res));
        }
        Command::Set { key, val, resp } => {
          let res = client.set(&key, val).await;
          // Ignore errors
          let _ = resp.unwrap().send(CmdResult::empty(res));
        }
      }
    }
  });

  // Spawn two tasks, one setting a value and other querying for key that was
  // set.
  let t1 = task(
    Command::Get {
      key: "foo".to_string(),
      resp: None,
    },
    tx,
  );
  let t2 = task(
    Command::Set {
      key: "foo".to_string(),
      val: "bar".into(),
      resp: None,
    },
    tx2,
  );

  t1.await;
  t2.await;
  manager.await.unwrap();
}
