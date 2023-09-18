use leet_code::leet_code::{array_to_tree, Solution};

#[test]
fn test_rob_198() {
    assert_eq!(Solution::rob_198(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_198(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn test_rob_213_v1() {
    assert_eq!(Solution::rob_213_v1(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob_213_v1(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_213_v1(vec![1, 2, 3]), 3);
}

#[test]
fn test_rob_213_v2() {
    assert_eq!(Solution::rob_213_v2(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob_213_v2(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_213_v2(vec![1, 2, 3]), 3);
}

#[test]
fn test_rob_337() {
    assert_eq!(
        Solution::rob_337(array_to_tree(vec![
            Some(3),
            Some(2),
            Some(3),
            None,
            Some(3),
            None,
            Some(1)
        ])),
        7
    );
}
