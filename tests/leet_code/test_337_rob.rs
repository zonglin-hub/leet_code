use leet_code::leet_code::{array_to_tree, Solution};

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
