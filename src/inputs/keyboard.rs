use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use crossterm::{event::{Event, KeyCode, KeyEventKind, poll, read}, terminal::enable_raw_mode};

pub fn spawn_throttle_input(throttle_target: Arc<AtomicU32>) {
    enable_raw_mode().unwrap();

    std::thread::spawn(move || {
        let mut pressed = false;
        let mut throttle: i32 = 0;

        loop {
            if poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    match event.code {
                        KeyCode::Char('a') => match event.kind {
                            KeyEventKind::Press => {
                                println!("pressed");
                                pressed = true;
                            }
                            KeyEventKind::Release => {
                                pressed = false;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            if pressed {
                throttle += 1;
                println!("pressed");
            } else {
                throttle -= 10;

            }

            throttle = throttle.clamp(0, 1000);

            throttle_target.store(throttle as u32, Ordering::Relaxed);

            println!("{}", throttle);

            std::thread::sleep(Duration::from_millis(16));
        }
    });
}
