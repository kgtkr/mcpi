extern crate rand;

fn main() {
    let mut i = 0i64;
    let mut c = 0;
    loop {
        let x = rand::random::<f64>();
        let y = rand::random::<f64>();
        if x * x + y * y < 1.0 {
            c += 1;
        }
        i += 1;
        if i % 1000000 == 0 {
            println!("{} {}", i, (c * 4) as f64 / i as f64);
        }
    }
}
