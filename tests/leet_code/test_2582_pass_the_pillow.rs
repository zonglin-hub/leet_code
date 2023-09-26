use leet_code::leet_code::Solution;

#[test]
fn test_largest_integer() {
    assert_eq!(Solution::pass_the_pillow(4, 5), 2);
    assert_eq!(Solution::pass_the_pillow(3, 2), 3);
}

#[test]
fn test_largest_integer_v1() {
    assert_eq!(Solution::pass_the_pillow_v1(4, 5), 2);
    assert_eq!(Solution::pass_the_pillow_v1(3, 2), 3);
}
