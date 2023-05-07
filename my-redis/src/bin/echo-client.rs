use std::error::Error;
use std::str::from_utf8;
use tokio::io::{AsyncWriteExt, self};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Connect to a peer
  let mut stream = TcpStream::connect("127.0.0.1:6142").await?;

  // Write some data.
  stream.write_all(b"hello world!").await?;
  loop {
    // Wait for the socket to be readable
    stream.readable().await?;

    // Creating the buffer **after** the `await` prevents it from
    // being stored in the async task.
    // let mut buf = [0; 100];
    let mut buf = Vec::with_capacity(1024);

    // Try to read data, this may still fail with `WouldBlock`
    // if the readiness event is a false positive.
    match stream.try_read_buf(&mut buf) {
      Ok(0) => break,
      Ok(n) => {
        println!("read {} bytes", n);
        println!("read data is {:?}", from_utf8(&buf));
      }
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        continue;
      }
      Err(e) => {
        return Err(e.into());
      }
    }
  }

  Ok(())
}
