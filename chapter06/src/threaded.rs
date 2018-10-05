use rand::prelude::*;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex, RwLock};

pub fn run() {
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let local_counter = Arc::clone(&counter);
        thread::spawn(move || {
            let wait = time::Duration::new(random::<u64>() % 8, 0);
            thread::sleep(wait);
            let mut shared = local_counter.lock().unwrap();
            *shared += 1;
        });
    }

    loop {
        {
            let shared = counter.lock().unwrap();

            println!("{} threads have completed", *shared);

            if *shared >= 10 {
                break;
            };
        };
        thread::sleep(time::Duration::new(1, 0));
    }
}
