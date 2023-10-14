use leet_code::leet_code::Solution;

#[test]
fn test_search_range() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

#[test]
fn test_partition_point() {
    let v = [1, 2, 3, 3, 5, 6, 7];
    let i = v.partition_point(|&x| x < 5);

    assert_eq!(i, 4);
    assert!(v[..i].iter().all(|&x| x < 5));
    assert!(v[i..].iter().all(|&x| !(x < 5)));
}
