use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let cv = Condvar::new();
    let data = Mutex::new(None);
    let pair = Arc::new((cv, data));

    for _ in 0..4 {
        let pair_clone = pair.clone();

        thread::spawn(move || {
            let (cv, data) = pair_clone.deref();

            loop {
                process_data(cv, data);
            }
        });
    }

    for i in 0..10 {
        let (cv, data) = pair.deref();
        *data.lock().unwrap() = Some(format!("some data {}", i));
        cv.notify_one();
        thread::sleep(Duration::from_secs_f32(0.25));
    }
}

fn process_data(cv: &Condvar, data: &Mutex<Option<String>>) {
    let thread_id = thread::current().id();
    let mut lock = data.lock().unwrap();

    while lock.is_none() {
        lock = cv.wait(lock).unwrap();
        println!("{:?} woken", thread_id);
    }

    println!("processing in {:?}: {}",  thread_id, lock.take().unwrap());
    thread::sleep(Duration::from_secs(1));
}
