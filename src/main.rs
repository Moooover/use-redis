use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
  // test
  let handle = tokio::spawn(async {
    println!("asdfasdf");
    let mut i = 0;
    while i < 1000000000 {
      i += 1;
    };
    "Hello World!"
  });
  let out = handle.await.unwrap();
  println!("JoinHandle out = {}", out);

  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
  loop {
    let (socket, _) = listener.accept().await.unwrap();
    tokio::spawn(async move {
      process(socket).await;
    });
  }
}

async fn process(socket: TcpStream) {
  let mut connection = Connection::new(socket);
  if let Some(frame) = connection.read_frame().await.unwrap() {
    println!("GOT: {:?}", frame);

    let response = Frame::Error("unimplemented".to_string());
    connection.write_frame(&response).await.unwrap();
  }
}