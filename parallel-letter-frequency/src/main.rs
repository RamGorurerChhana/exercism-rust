// use std::{collections::HashMap, sync::Mutex};

// use crossbeam::thread;  

// fn main() {
//     let input = &["This is a message", "This is a second message", "and so on and on.."];
//     let worker_count = 3;
//     let counter = Mutex::new(HashMap::new());

//     for worker_no in 0..worker_count {
//         // let counter = Arc::clone(&counter);
//         thread::scope(|s| {
//             let handle = s.spawn(|_| {
//                 let mut i = worker_no;
//                 loop {
//                     if i >= input.len() {
//                         break;
//                     }
//                     println!("i is {i}, worker_no is {worker_no}");
//                     let mut counter = counter.lock().unwrap();
//                     counter.entry('a').and_modify(|count| *count += 1).or_insert(1);
//                     i += worker_no;
//                 }


//                 // while let Some(ss) = input.get(i) {
//                 //     ss.chars().filter(|ch| ch.is_alphabetic()).for_each(|ch|{
//                 //         let ch = if ch.is_uppercase() {ch.to_lowercase().next().unwrap()} else {ch};
//                 //         let mut counter = counter.lock().unwrap();
//                 //         counter.entry(ch).and_modify(|count| *count += 1).or_insert(1);
//                 //     });
//                 //     i += worker_no;
//                 // }
//             });
//             handle.join().unwrap();
//         }).unwrap();
//     }
//     println!("{:?}", counter);
// }




use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}
