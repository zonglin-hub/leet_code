use leet_code::leet_code::Solution;

#[test]
fn test_split_num() {
    assert_eq!(Solution::split_num(4325), 59);
    assert_eq!(Solution::split_num(687), 75);
}

#[test]
fn test_split_num_v1() {
    assert_eq!(Solution::split_num_v1(4325), 59);
    assert_eq!(Solution::split_num_v1(687), 75);
}

#[test]
fn test_split_num_v2() {
    assert_eq!(Solution::split_num_v2(4325), 59);
    assert_eq!(Solution::split_num_v2(687), 75);
}
