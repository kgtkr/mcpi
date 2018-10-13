extern crate rand;

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut i = 0;
            let mut c = 0;
            loop {
                let x = rand::random::<f64>();
                let y = rand::random::<f64>();
                if x * x + y * y < 1.0 {
                    c += 1;
                }
                i += 1;
                if i % 1000000 == 0 {
                    tx.send((i, c)).unwrap();
                    i = 0;
                    c = 0;
                }
            }
        });
    }

    let mut i = 0;
    let mut c = 0;
    loop {
        let (x, y) = rx.recv().unwrap();
        i += x;
        c += y;
        println!("{} {}", i, (c * 4) as f64 / i as f64);
    }
}
