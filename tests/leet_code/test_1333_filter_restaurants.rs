use leet_code::leet_code::Solution;

#[test]
fn test_filter_restaurants() {
    let rest = vec![
        vec![1, 4, 1, 40, 10],
        vec![2, 8, 0, 50, 5],
        vec![3, 8, 1, 30, 4],
        vec![4, 10, 0, 10, 3],
        vec![5, 1, 1, 15, 1],
    ];

    let mut res = vec![3, 1, 5];
    res.sort();
    assert_eq!(Solution::filter_restaurants(rest.clone(), 1, 50, 10), res);

    let mut res = vec![4, 3, 2, 1, 5];
    res.sort();
    assert_eq!(Solution::filter_restaurants(rest.clone(), 0, 50, 10), res);

    let mut res = vec![4, 5];
    res.sort();
    assert_eq!(Solution::filter_restaurants(rest.clone(), 0, 30, 3), res);
}
