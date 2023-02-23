use std::thread;
use std::time::Duration;

pub fn demo() {
    move_var();
    loop_two_threads();
}

fn move_var() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("There is a vector: {:?}", v)); //force taking ownership

    //join will make sure the thread finishes before the main thread shuts down
    handle.join().unwrap();
}

fn loop_two_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread {}", i);
            //sleep to let the other thread run
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
