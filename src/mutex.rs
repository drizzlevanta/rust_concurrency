use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn demo() {
    demo_mutex();
    multi_ownership();
}

fn demo_mutex() {
    let m = Mutex::new(5);

    {
        let mut number = m.lock().unwrap();
        *number = 7;
    }

    println!("m is now {:?}", m);
}

fn multi_ownership() {
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let count = count.clone(); //multiple ownership
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // handles.iter().map(|h|{

    // })
    //IMPT into_iter instead of iter will consume the vector, and will get ownership on each element
    // handles.into_iter()(|h| h.join().unwrap()).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Calculation in {:?} threads", *count.lock().unwrap());
}
