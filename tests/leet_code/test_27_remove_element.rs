use leet_code::Solution;

#[test]
fn test() {
    assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(
        Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
        5
    );
}
