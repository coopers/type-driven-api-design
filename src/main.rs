use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn progress<T>(v: Vec<T>, f: fn(&T) -> ()) {
    let mut i = 1;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    progress(v, expensive_calculation);
}
