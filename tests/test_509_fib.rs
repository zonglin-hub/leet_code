use leet_code::Solution;

#[test]
fn test_fib_v1() {
    assert_eq!(Solution::fib_v1(2), 1);
    assert_eq!(Solution::fib_v1(3), 2);
    assert_eq!(Solution::fib_v1(4), 3);
}

#[test]
fn test_fib_v2() {
    assert_eq!(Solution::fib_v2(2), 1);
    assert_eq!(Solution::fib_v2(3), 2);
    assert_eq!(Solution::fib_v2(4), 3);
}
