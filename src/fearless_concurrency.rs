pub mod use_fearless_concurrency {
    use std::rc::Rc;
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub fn build() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1)); // forces a thread to stop its execution for a short duration, allowing a different thread to run
            }
        });

        // handle.join().unwrap(); // The main thread will wait for the spawned thread to finish and then run its for loop

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // blocks the thread currently running until the thread represented by the handle terminates
        // Blocking a thread means that thread is prevented from performing work or exiting.

        let v = vec![1,2,3];

        let handle1 = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        handle1.join().unwrap();

        // Message passing, where threads or actors communicate by sending each other messages containing data.
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        thread::spawn(move || { // transmitter
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1)); // 1s pause after sending a message
            }
        });

        thread::spawn(move || { // transmitter clone, runs simultaneously with the original
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }

        shared_state_concurrency();
    }

    fn shared_state_concurrency() {
        // "Do not communicate by sharing memory."
        // mutex stands for mutual exclusion - mutex allows only one thread to access some data at any given time
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {m:?}");

        // Multiple Ownership with Multiple Threads
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
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}