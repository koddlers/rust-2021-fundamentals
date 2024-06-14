pub mod concurrency_and_rust {
    use std::thread;
    use std::time::Duration;

    pub fn using_threads() {
        thread::spawn(|| {
            println!("This log is coming from within a separate thread.");
        });

        // this line will `almost` always print first
        println!("This log is coming from within the `main` thread");

        // joining threads
        let thread_handle = thread::spawn(|| {
            for num in 1..=30 {
                println!("This log is coming from a separate thread within the for loop: {}", num);
                thread::sleep(Duration::from_millis(100));
            }
        });

        // joining threads in `thread_handle`
        thread_handle.join().unwrap();

        // transferring values to threads using "move closures"
        let my_str = String::from("Hello World");

        let handle = thread::spawn(move || {
            println!("This thread now owns this string: {}", my_str);
        });
        handle.join().unwrap();
    }
}