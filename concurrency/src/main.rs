use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for x in 1..10{
            println!("number {} from spawned thread", x);
            thread::sleep (Duration::from_millis(500))
        }
    });

    vec![1,2,3,4,5].into_iter()
        .for_each(|x| {
            println!("number {} from main thread", x);
            thread::sleep (Duration::from_millis(700))
        });


    // Block main thread until spawn thread finishes
    handle.join();

    println!("Finished printing numbers\n");

    let v = vec![1, 2, 3];

    // Instead of passing a ref of v to the thread, you move ownership of v to the thread
    // move will move any referenced variables in the thread to it's ownership
    let handle2 = thread::spawn(move || {
        println!("Here's the vector 1 2 3: {:?}", v);
    });

    handle2.join();


    println!("\nMESSAGE PASSING\n");

    // mpsc - Multiple Producer, Single Consumuer
    let (tx, rx) = mpsc::channel();

    // Moving ownership of tx to the thread
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    // recv will block the main thread until it receives it's message
    let msg_received = rx.recv().unwrap();
    println!("Received message: {}\n\n", msg_received);

    let (tx2, rx2) = mpsc::channel();

    // Cloning Sender
    let tx2_clone = mpsc::Sender::clone(&tx2);

    thread::spawn(move || {
        let msgs = vec!["hello1".to_string(), "hello2".to_string(), "hello3".to_string(),
                        "hello4".to_string(), "hello5".to_string()];

        msgs.into_iter().for_each(|msg| {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1)); // to better see the messages being received
        });
    });

    thread::spawn(move || {
        let msgs = vec!["goodbye1".to_string(), "goodbye2".to_string(), "goodbye3".to_string(),
                        "goodbye4".to_string(), "goodbye5".to_string()];

        msgs.into_iter().for_each(|msg| {
            tx2_clone.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1)); // to better see the messages being received
        });
    });

    rx2.into_iter().for_each(|msg| println!("Received message: {}", msg));
}
