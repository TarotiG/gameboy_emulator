use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel();
    let frame_counter = Arc::new(Mutex::new(0));
    let rendered_frame_counter = Arc::clone(&frame_counter);

    // CPU thread
    let cpu = thread::spawn(move || {

        for i in 1..=10 {
            let frame = vec![0; 5];
            tx.send(frame).unwrap();
            thread::sleep(Duration::from_millis(16));
        }
    });

    // Render thread
    let render = thread::spawn(move || {
        for received_frame in rx {
            let mut counter = rendered_frame_counter.lock().unwrap();
            *counter += 1;
            println!("Received frame: {} Data: {:?}", counter, received_frame);
        }
    });

    cpu.join().unwrap();
    render.join().unwrap();
}
