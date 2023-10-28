use leet_code::leet_code::Solution;

#[test]
fn test_55_can_jump() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![vec![1, 5], vec![6, 9]]
    );
    assert_eq!(
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
    assert_eq!(
        Solution::insert(Vec::<Vec<i32>>::new(), vec![5, 7]),
        vec![vec![5, 7]]
    );
    assert_eq!(
        Solution::insert(vec![vec![1, 5]], vec![2, 3]),
        vec![vec![1, 5]]
    );
    assert_eq!(
        Solution::insert(vec![vec![1, 5]], vec![2, 7]),
        vec![vec![1, 7]]
    );
}
