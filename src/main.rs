//#![allow(unused)]

use std::thread;

const STACK_SIZE: usize = 2 * 1024 * 1024 * 1024; // стек пару гигабайт
static mut COUNTER: i128 = 0;

fn foo(n: u64) -> u64 {
    if n > 0 {
        unsafe {
            COUNTER += 1;
        }
        foo(n - 1);
    }
    0
}

fn test_recurse() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            unsafe {
                println!("counter {}", COUNTER);
            }
            foo(40_500_000);
            unsafe {
                println!("counter {}", COUNTER);
            }
        })
        .unwrap();
    child.join().unwrap();
}

fn main() {
    test_recurse();
}
