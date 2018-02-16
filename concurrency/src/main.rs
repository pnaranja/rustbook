use std::thread;

fn main() {
    let handle = thread::spawn(|| {
//        vec![1..10].into_iter()
//            .for_each(|x| println!("number {:?} from spawned thread", x))
        for x in 1..10{
            println!("number {} from spawned thread", x);
        }
    });

    vec![1,2,3,4,5].into_iter()
        .for_each(|x| println!("number {} from main thread", x));

    // Block main thread until spawn thread finishes
    handle.join();

    println!("Program finished");
}
