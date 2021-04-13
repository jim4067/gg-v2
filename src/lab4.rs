//James Mutuku I44/2397/2019

#![allow(dead_code)]

fn main() {}

//converting temp from degrees celsius to fahrenheit
//(°C × 9/5) + 32 = °F
fn degrees_to_fahrenheit(deg_cel: i64) -> f64 {
    let res = deg_cel as f64 * 1.8 + 32.0;
    res
}

#[test]
fn degrees_to_fahrenheit_test() {
    assert_eq!(degrees_to_fahrenheit(23), 73.4);
}

//generating fibonacci numbers
fn fib(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => fib(num - 1) + fib(num - 2),
    }
}

#[test]
fn fib_is_8() {
    assert_eq!(fib(5), 8);
}