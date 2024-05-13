use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use tokio::{self, time::sleep};

#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        enigo.key(Key::Meta, Direction::Click).unwrap();
        sleep(Duration::from_secs(1500)).await;
    }
}
