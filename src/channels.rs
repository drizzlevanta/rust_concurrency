use std::{sync::mpsc, thread};

use std::time::Duration;

pub fn demo() {
    send_rev();
    send_rev_multi();
    multi_producer_single_consumer();
}

fn send_rev() {
    let (tx, rx) = mpsc::channel();

    //send msg
    thread::spawn(move || {
        let val = String::from("Hi");
        // let val = "hi";
        tx.send(val).unwrap();
    });

    //consume msg
    //recv() will block the main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_rev_multi() {
    let (tx, rx) = mpsc::channel();
    // let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // send multiple msg
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("today"),
            String::from("is"),
            String::from("sunny"),
        ];

        // vals.iter().map(|s|{

        // })
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //receive multiple msg
    for r in rx {
        println!("Got {}", r);
    }

    println!("end of main thread");
}

fn multi_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    // send multiple msg
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("today"),
            String::from("is"),
            String::from("sunny"),
        ];

        for val in vals {
            tx.send(val).unwrap(); //move ownership of val
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Here"),
            String::from("more"),
            String::from("msgs"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
        }
    });

    //receive multiple msg
    for r in rx {
        println!("Got {}", r);
    }

    println!("end of main thread");
}
