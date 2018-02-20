use std::sync::{Mutex, Arc};
use std::sync::MutexGuard;
use std::thread;

fn main() {
    let num = Mutex::new(5);

    {
        // In order to get access to num, you need to lock the Mutex
        // Create a MutexGuard to lock num
        // lock() -> LockGuard<MutexGuard<T>>
        let mut num_lock: MutexGuard<i32> = num.lock().unwrap();
        *num_lock = 7;
    } // <-- Drop is called on the MutexGuard.  This releases the Mutex lock

    println!("New num is {:?}", num.into_inner().unwrap());


    // Arc is Rc but thread safe
    let count_mae = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&count_mae);
        // Create the thread
        let handle = thread::spawn(move || {
            let mut num_mae = counter.lock().unwrap();
            *num_mae += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The count_mae is {}", *count_mae.lock().unwrap());
}
