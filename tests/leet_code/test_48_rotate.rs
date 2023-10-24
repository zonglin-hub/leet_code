use leet_code::leet_code::Solution;

#[test]
fn test_rotate() {
    let mut t_vec1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut t_vec1);
    assert_eq!(t_vec1, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

    let mut t_vec2 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut t_vec2);
    assert_eq!(
        t_vec2,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );
}
