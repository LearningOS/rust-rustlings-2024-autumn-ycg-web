// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

struct Queue {
    sa
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc = Arc::new(q);

    // Spawn first thread to send the first_half
    let handle1 = thread::spawn({
        let tx = Arc::clone(&tx);
        let qc1 = Arc::clone(&qc);
        move || {
            for val in &qc1.first_half {
                println!("sending {:?}", val);
                let tx = tx.lock().unwrap();  // Lock the tx before sending
                tx.send(*val).unwrap();
            }
        }
    });

    // Spawn second thread to send the second_half
    let handle2 = thread::spawn({
        let tx = Arc::clone(&tx);
        let qc2 = Arc::clone(&qc);
        move || {
            for val in &qc2.second_half {
                println!("sending {:?}", val);
                let tx = tx.lock().unwrap();  // Lock the tx before sending
                tx.send(*val).unwrap();
            }
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));  // Wrap tx in an Arc and Mutex
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    // Receive all data
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("Total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length); // Assert that the total received matches the expected length
}
