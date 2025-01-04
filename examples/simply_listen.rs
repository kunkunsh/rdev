use rdev::{listen, Event, EventType};

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(_key) | EventType::KeyRelease(_key) => {
            println!("User wrote {:?}", event.unicode);
        }
        _ => (),
    }
}

fn main() {
    // This will block.
    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
    thread::sleep(Duration::from_secs(5));
    rdev::stop_listen().unwrap();
    let _ = handle.join();
    println!("Done");
}
