// //Assignment 3: Implementing a Thread Pool in Rust
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// // Message to be sent to the workers
// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// // Job type is a boxed closure that can be sent across threads
// type Job = Box<dyn FnOnce() + Send + 'static>;

// // ThreadPool struct
// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     // Create a new ThreadPool with the specified size
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));

//         let workers = (0..size)
//             .map(|id| Worker::new(id, Arc::clone(&receiver)))
//             .collect();

//         ThreadPool { workers, sender }
//     }
    
    
//     // Execute a job in the thread pool
//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         // TODO: Create a job from the closure and send it to a worker
//         let job = Box::new(f);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// // Clean up resources when ThreadPool is dropped
// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         // TODO: Send terminate message to all workers
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         // TODO: Wait for all workers to finish
//         for worker in &mut self.workers {
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// // Worker struct represents a thread that can process jobs
// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     // Create a new worker with the specified ID
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         // TODO: Create a thread that loops and receives jobs from the channel
//         let thread = thread::spawn(move || loop {
//             let message = receiver.lock().unwrap().recv().unwrap();

//             match message {
//                 Message::NewJob(job) => {
//                     println!("Worker {} got a job; executing.", id);
//                     job(); 
//                 }
//                 Message::Terminate => {
//                     println!("Worker {} was told to terminate.", id);
//                     break;
//                 }
//             }
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// fn main() {
//     // Create a new thread pool with 4 workers
//     let pool = ThreadPool::new(4);
    
//     // Submit 10 tasks to the pool
//     for i in 1..=10 {
//         pool.execute(move || {
//             println!("Processing task {}", i);
//             thread::sleep(std::time::Duration::from_millis(500));
//             println!("Completed task {}", i);
//         });
//     }
    
//     println!("Main thread waiting for tasks to complete...");
//     // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
// }

//Assignment 4: Implementing a Producer-Consumer Pattern in Rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{rng, RngExt};

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(rx));

    // TODO: Create 2 producer threads
    let producer_handles: Vec<_> = (0..2).map(|id| {
        let tx_clone = tx.clone();
        thread::spawn(move || producer(id, tx_clone, ITEM_COUNT))
    }).collect();
    
    // TODO: Create 3 consumer threads
    let consumer_handles: Vec<_> = (0..3).map(|id| {
        let rx_clone = receiver.clone();
        thread::spawn(move || consumer(id, rx_clone))
    }).collect();
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function


fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rng();

    for _ in 0..item_count {
        let num = rng.random_range(1..100);
        println!("Producer {} produced: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        if num == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        }
        println!("Consumer {} consumed: {}", id, num);
        thread::sleep(Duration::from_millis(150));
    }
}