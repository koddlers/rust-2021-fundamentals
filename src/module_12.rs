pub mod concurrency_and_rust {
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use std::thread::JoinHandle;
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

    pub fn message_passing_using_channels() {
        // MPSC: Multiple Producers, Single Consumer
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let my_str = String::from("Hello from other thread");
            tx.send(my_str).unwrap();
        });

        let received_msg = rx.recv().unwrap();
        println!("Got this message from a different thread: {}", received_msg);
    }

    pub fn shared_state_concurrency() {
        let my_mutex = Mutex::new(42);

        {
            let mut custom_data_ref = my_mutex.lock().unwrap();
            *custom_data_ref = 12;
        }

        // the lock is released automatically when the mutex goes out of scope
    }

    #[derive(Debug)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    pub fn demo_threads_and_channels() {
        // Spawning threads
        let thread_one = thread::spawn(|| {
            println!("Logging from Thread 1");
        });

        // We can use the thread "builder" to build a new thread with a name
        let thread_two =
            thread::Builder::new().name("Thread 2".to_string()).spawn(|| {
                println!("Logging from Thread 2");
            }).unwrap();

        // This thread will not complete until Thread 2 completes
        let thread_three = thread::spawn(|| {
            // Capture the underlying Thread object
            let two = thread_two.thread();
            println!("Thread 2 name: {:?} and id: {:?}", two.name(), two.id());

            thread_two.join().unwrap();
            println!("Logging from Thread 3");
        });

        // Joining thread handles (waiting for threads to finish)
        // Any code after these 2 lines will not execute until are threads are complete
        thread_one.join().unwrap();
        thread_three.join().unwrap();

        // Message passing between threads using channels
        let (tx, rx) = mpsc::channel();

        // Not capturing the thread handle just causes these threads to kick off - main thread
        // doesn't have to wait for them to finish
        thread::spawn(move || {
            for id in 0..20 {
                let coffee = Coffee { id: id + 1, count: 50 + id };
                tx.send(coffee).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        let rx = thread::spawn(move || {
            for _ in 0..20 {
                let coffee = rx.recv().unwrap();
                println!("Received coffee: {:?}", coffee);
                thread::sleep(Duration::from_millis(750));
            }
        });
        rx.join().unwrap();

        // Shared-state between threads using mutexes
        // Here we use an Arc (short for Atomic reference counter) so that we can
        // safely have shared ownership of the mutex between threads
        let message = Arc::new(Mutex::from(String::new()));

        // This vector helps us keep track of threads that we spawn within the main thread so that
        // we can wait for them all to finish
        let mut thread_handles: Vec<JoinHandle<()>> = vec![];

        // Spawn 5 threads - have each thread concurrently mutate our shared String
        for num in 0..5 {
            // Clone our atomic primitive - Arcs can safely be shared between threads
            let message = Arc::clone(&message);

            let current_handle = thread::spawn(move || {
                // Within the closure that each thread runs, lock our message so that other
                // threads can't mutate it while this thread is...
                let mut current_message = message.lock().unwrap();

                // Mutate the string
                let mut base_str = String::from(" Thread: ");
                base_str.push_str((num + 1).to_string().as_str());

                current_message.push_str(&base_str);
                println!("Message after thread altered it: {}", current_message);
            });

            thread_handles.push(current_handle);
        }

        // Wait for all threads to finish mutating our shared-state
        for handle in thread_handles {
            handle.join().unwrap();
        }

        println!("Final message: {}", message.lock().unwrap());
    }

    // This struct implicitly implements "Send" because the i32 and String type is Send
    // This struct also implicitly implements "Sync" for the same reason.
    #[derive(Debug)]
    struct Kofi {
        id: i32,
        count: i32,
        name: String,
    }

    pub fn demo_using_the_sync_and_send_traits() {
        // Message passing between threads using channels
        let (tx, rx) = mpsc::channel();

        // Not capturing the thread handle just causes these threads to kick off - main thread
        // doesn't have to wait for them to finish
        thread::spawn(move || {
            for id in 0..20 {
                let kofi = Kofi {
                    id: id + 1,
                    count: 50 + id,
                    name: String::from("Drip ".to_owned() + &(id + 1).to_string()),
                };
                // We can send coffees here because each field within Coffee is Send
                // Send let's us transfer ownership between threads
                tx.send(kofi).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        let rx = thread::spawn(move || {
            for _ in 0..20 {
                let kofi = rx.recv().unwrap();
                println!("Received Kofi: {:?}", kofi);
                thread::sleep(Duration::from_millis(750));
            }
        });
        rx.join().unwrap();

        // Kofi, like Arc implements Sync and can safely be given access from multiple threads
        let drip_kofi = Kofi { id: 0, count: 20, name: String::from("Drip Kofi") };
        let kofi = Arc::new(Mutex::from(drip_kofi));

        let mut thread_handles: Vec<JoinHandle<()>> = vec![];
        for _ in 0..5 {
            let kofi = Arc::clone(&kofi);

            let current_handle = thread::spawn(move || {
                let mut kofi = kofi.lock().unwrap();

                kofi.count += 5;
                println!("Kofi count after thread altered it: {}", kofi.count);
            });
            thread_handles.push(current_handle);
        }

        // Wait for all threads to finish mutating our shared-state
        for handle in thread_handles {
            handle.join().unwrap();
        }

        println!("Final count: {}", kofi.lock().unwrap().count);

        // Final Note:
        // You *can* implement Sync and Send traits manually but this is UNSAFE
        // You don't have to derive these directly since these traits are automatically
        // implemented when all sub-types implement them.
    }
}