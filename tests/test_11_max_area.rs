use leet_code::Solution;

#[test]
fn test_max_area_v1() {
    assert_eq!(Solution::max_area_v1(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area_v1(vec![1, 1]), 1);
}
