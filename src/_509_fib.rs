#![allow(unused)]
struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |mut t, _| (t.1, t.0 + t.1)).0
    }

    pub fn fib_1(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            _ => Self::fib_1(n - 2) + Self::fib_1(n - 1),
        }
    }
}
