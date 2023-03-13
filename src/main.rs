use std::env;
use serde::{Deserialize, Serialize};
use serde_json_wasm;

#[derive(Serialize, Deserialize)]
struct CustomOption {
    n: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let v: CustomOption = serde_json_wasm::from_str(&args[1]).unwrap();

    let n:i32 = v.n;

    // 한글 깨짐이 있음
    println!("{}st fibonacci is {}", n, fibonacci_reccursive(n));
}

pub fn fibonacci_reccursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 => 0,
        2 | 3 => 1,
        /*
        50    => 12586269025,
        */
        _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
    }
}