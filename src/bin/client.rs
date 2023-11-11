use mini_redis::client;
use bytes::Bytes;

#[derive(Debug)]
enum Command {
  Get {
    key: String,
  },
  Set {
    key: String,
    val: Bytes
  }
}

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {

  let (tx, mut rx) = mpsc::channel(32);
  let manager = tokio::spawn(async move {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(cmd) = rx.recv().await {
      use Command::*;

      match cmd {
        Get { key } => {
          client.get(&key).await;
        }
        Set { key, val } => {
          client.set(&key, val).await;
        }
      }
    }
  });

  let tx2 = tx.clone();

  tokio::spawn(async move {
    let cmd = Command::Get {
      key: "foo".to_string()
    };
    tx.send(cmd).await.unwrap();
  });

  tokio::spawn(async move {
    let cmd = Command::Set {
      key: "foo".to_string(),
      val: "bar".into()
    };
    tx2.send(cmd).await.unwrap();
  });
}