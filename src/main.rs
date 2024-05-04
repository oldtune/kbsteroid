use std::{thread, time::Duration};

use enigo::{Coordinate, Enigo, Keyboard, Mouse, Settings};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        enigo.move_mouse(500, 200, Coordinate::Abs).unwrap();
        enigo.move_mouse(200, 500, Coordinate::Abs).unwrap();
        thread::sleep(Duration::from_secs(5));
        enigo.text("Hello world").unwrap();
    }
}
