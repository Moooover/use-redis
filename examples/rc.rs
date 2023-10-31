use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  tokio::spawn(async {
    {
      let rc = Rc::new("hello");
      println!("rc {}", rc);
    }
    yield_now().await;
  });
}