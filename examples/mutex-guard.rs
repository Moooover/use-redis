use std::sync::{ Mutex, MutexGuard };

struct CanIncrement {
  mutex: Mutex<i32>
}

impl CanIncrement {
  fn increment(&self) {
    let mut lock = self.mutex.lock().unwrap();
    *lock += 1;
  }
}

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
  let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
  *lock += 1;
}

async fn increment_and_do_stuff_can_increment(can_incr: &CanIncrement) {
  can_incr.increment();

}

fn main() {
  
}