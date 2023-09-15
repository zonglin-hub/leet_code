use leet_code::Solution;

#[test]
fn test() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert_1(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert_2(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert_3(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert_4(vec![1, 3, 5, 6], 7), 4);
}
