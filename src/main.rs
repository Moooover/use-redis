use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

use tokio::task::yield_now;
use std::rc::Rc;

use bytes::Bytes;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

const SHARD_COUNT: usize = 5;
#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

  let db = new_sharded_db(SHARD_COUNT); // Arc::new(Mutex::new(HashMap::new()));

  loop {
    let (socket, _) = listener.accept().await.unwrap();

    let db = db.clone();

    tokio::spawn(async move {
      process(socket, db).await;
    });
  }
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
  let mut db = Vec::with_capacity(num_shards);
  for _ in 0..num_shards {
    db.push(Mutex::new(HashMap::new()))
  }
  Arc::new(db)
}

fn hash(key: String) -> usize {
  key.len()
}

async fn process(socket: TcpStream, db: ShardedDb) {

  use mini_redis::Command::{self, Get, Set};

  // handles parsing frames from the socket
  let mut connection = Connection::new(socket);

  // Use `read_frame` to receive a command from the connection
  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
        let mut shard = db[hash(cmd.key().to_string()) % SHARD_COUNT].lock().unwrap();
        shard.insert(cmd.key().to_string(), cmd.value().clone());
        Frame::Simple("OK".to_string())
      }
      Get(cmd) => {
        let shard = db[hash(cmd.key().to_string()) % SHARD_COUNT].lock().unwrap();
        if let Some(value) = shard.get(cmd.key()) {
          Frame::Bulk(value.clone().into())
        } else {
          Frame::Null
        }
      }
      cmd => panic!("unimplemented {:?}", cmd),
    };
    connection.write_frame(&response).await.unwrap();
  }  
}