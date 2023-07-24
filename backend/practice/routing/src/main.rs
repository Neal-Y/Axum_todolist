use routing::run;

#[tokio::main]
async fn main() {
    run().await;
}

// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::thread;

// static GLOBAL_DATA: AtomicUsize = AtomicUsize::new(0);

// fn main() {
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(|| {
//             for _ in 0..500 {
//                 GLOBAL_DATA.fetch_add(1, Ordering::SeqCst);
//             }
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final value: {}", GLOBAL_DATA.load(Ordering::SeqCst));
// }
