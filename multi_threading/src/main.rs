use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;
use std::sync::Arc;
use futures::executor::block_on;
use std::thread::JoinHandle;


fn main() {
    // multi1();
    // multi2();
    // channel1();
    // mutex1();
    // mutex2();
    async1();
}

fn multi1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the child thread!", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("number {} from the main", i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn multi2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}

fn channel1() {
    let (sender, receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        // let val = String::from("hi");
        // sender.send(val).unwrap();

        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("child"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(500))

        }
    });

    // let received = receiver.recv().unwrap();
    // println!("Got: {}", received);

    for received in receiver {
        println!("Got: {}", received);
    }
}

fn mutex1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    println!("m = {:?}", m)
}

fn mutex2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn async1() {
    println!("main thread. id: {:?}", thread::current().id());
    // print_in_main_thread();

    let th_eat = eat_in_thread();
    let th_play = play_in_thread();

    th_eat.join().unwrap();
    th_play.join().unwrap();

    std::thread::spawn(move || {
        println!("Print from other thread");
    });

    task_print()
}



fn eat_in_thread() -> JoinHandle<()> {
    std::thread::spawn(move || {
        let current_id = thread::current().id();

        for i in 0..5 {
            println!("eat -> thrad_id: {:?}", current_id);
            std::thread::sleep(Duration::from_millis(500));
            println!("eat -> thread_id: {:?}", current_id);
        }
    });
}

fn play_in_thread() -> JoinHandle<()> {
    std::thread::spawn(move || {
        let current_id = thread::current().id();

        for i in 0..5 {
            println!("play -> thread_id: {:?}", current_id);
            std::thread::sleep(Duration::from_millis(500));
            println!("play -> thread_id: {:?}", current_id);
        }
    });
}

