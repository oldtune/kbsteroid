use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};
use rand::prelude::*;
use std::time::Duration;
use tokio::{self, time::sleep};

#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut rng = thread_rng();
    let mut total = 0;
    let mut tab_index = 1;
    const RADIX: u32 = 10;

    loop {
        // if rand::random() {
        //     enigo.key(Key::Meta, Direction::Click).unwrap();
        //     enigo.key(Key::Meta, Direction::Click).unwrap();
        // }

        if rand::random() {
            enigo.move_mouse(0, 0, enigo::Coordinate::Rel).unwrap();
        }

        if rand::random() {
            enigo.scroll(1, enigo::Axis::Vertical).unwrap();
        }

        let jitter = rng.gen_range(1000..=1500);
        total += jitter;
        sleep(Duration::from_millis(jitter)).await;

        if total >= 300_000 {
            total = 0;
            let tab_index_as_char = char::from_digit(tab_index, RADIX).unwrap();

            enigo.key(Key::Option, Direction::Press).unwrap();
            enigo
                .key(Key::Unicode(tab_index_as_char), Direction::Click)
                .unwrap();
            enigo.key(Key::Control, Direction::Release).unwrap();

            if tab_index < 9 {
                tab_index = tab_index + 1;
            } else {
                tab_index = 1;
            }
        }
    }
}
