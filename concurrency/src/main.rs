// use std::rc::Rc; // reference counting types, here replaced by using Arc for thread-safety (~ atomicity)
// use std::sync::Arc;
// use std::sync::mpsc; // channel (i.e. multiple producer, single consumer)
// use std::sync::Mutex; // has to be locked before access to inner value and returns a LockResult<MutexGuard<'_, T>>
use std::sync::{Arc, mpsc, Mutex}; // the elegant way to import the types
use std::thread;
use std::time::Duration;

fn main() {
    using_threads();
    using_messaging();
    using_shared_state_concurrency();
}

fn using_threads(){
    //thread::spawn(|| {
    let handle = thread::spawn(|| { // returns a JoinHandle
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // blocks the thread currently running until the thread represented by the handle terminates

    let v = vec![1, 2, 3];

    //let handle = thread::spawn(|| { // error: closure may outlive function from which v is borrowed
    let handle = thread::spawn(move || { // force the closure to take ownership of the values it’s using
                                            // rather than allowing Rust to infer that it should borrow the values
        println!("Here's a vector: {:?}", v);
    });

    //drop(v); // won't work, because v is moved while being borrowed
                // move doesn't help either, because here we use a variable áfter it has been moved

    handle.join().unwrap();

}

fn using_messaging() {
    let (tx, rx) = mpsc::channel(); // returns (Sender<T>, Receiver<T>); this let statement destructures the tuple

    thread::spawn(move || { // move tx into closure, to enable spawned thread to send messages
        let val = String::from("hi from spawn");
        tx.send(val).unwrap(); // send returns a Result<()), SendError>; unwrap will panic in case of error (e.g. rx has been dropped)
        // println!("val is {}", val); // error: value being borrowed after move
    });

    // recv() will block the main thread’s execution and wait until a value is sent down the channel
    let received = rx.recv().unwrap(); // recv (receive) returns a result<T, RecvError>
    println!("Got: {}", received);
    // try_recv() will not block but return immediately with either OK or TryRecvError

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // rx can be treated as an iterator!
        println!("Got: {}", received);
    }

    // Creating Multiple Producers by Cloning the Transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
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

    for received in rx { // message are received in the order they were sent
        println!("Got: {}", received);
    }
}

fn using_shared_state_concurrency(){
    //let counter = Mutex::new(0); // counter is 'guarded' by a Mutex<i32>, but multiple ownership is not possible
    //let counter = Rc::new(Mutex::new(0)); // enable multiple ownership, but Rc is not thread-safe, i.e. does not implement std::marker::Send (needed by closure in thread::spawn)
    let counter = Arc::new(Mutex::new(0)); // atomically reference counted; see std::sync::atomic (primitive types that can be shared across threads)
    let mut handles = vec![];

    for _ in 0..10 { // spawn 10 threads that increment the counter by 1
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // unwrap to get access to T (i32)

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { // collect all join handles
        handle.join().unwrap(); // make sure all threads finish
    }

    println!("Result: {}", *counter.lock().unwrap()); // acquire lock on counter and print result
}
