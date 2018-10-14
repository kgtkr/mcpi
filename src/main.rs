#[macro_use]
extern crate clap;
extern crate rand;
extern crate separator;
use separator::Separatable;

use clap::{App, Arg};
use std::sync::mpsc;
use std::thread;

fn main() {
    let app = App::new("MCPI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("tkr <kgtkr.jp@gmail.com>")
        .about("モンテカルロ法で円周率計算")
        .arg(
            Arg::with_name("threads_count")
                .help("スレッド数")
                .long("threads")
                .short("t")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("log")
                .help("何回に一回ログを出力するか")
                .long("log")
                .short("l")
                .takes_value(true)
                .default_value("1000000000"),
        );

    let matches = app.get_matches();
    let threads_count = value_t!(matches, "threads_count", i32).unwrap_or_else(|e| e.exit());
    let log = value_t!(matches, "log", i64).unwrap_or_else(|e| e.exit());

    let (tx, rx) = mpsc::channel();
    for _ in 0..threads_count {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut i = 0i64;
            let mut c = 0i64;
            loop {
                let x = rand::random::<f64>();
                let y = rand::random::<f64>();
                if x * x + y * y < 1.0 {
                    c += 1;
                }
                i += 1;
                if i % log == 0 {
                    tx.send((i, c)).unwrap();
                    i = 0;
                    c = 0;
                }
            }
        });
    }

    let mut i = 0i64;
    let mut c = 0i64;
    loop {
        let (x, y) = rx.recv().unwrap();
        i += x;
        c += y;
        println!("{:>16} {}", i.separated_string(), (c * 4) as f64 / i as f64);
    }
}
