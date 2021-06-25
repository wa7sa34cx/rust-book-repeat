use std::thread;
use std::time::Duration;

pub fn run() {
    let x = 10;

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread, x = {}", i, x);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // thread::sleep(Duration::from_millis(10));
}
