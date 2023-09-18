use leet_code::leet_code::{expected_sort, expected_sort_vec, Solution};

#[test]
fn test_queens_attackthe_king_v1() {
    assert_eq!(
        expected_sort(Solution::queens_attackthe_king_v1(
            expected_sort_vec(vec![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]]),
            vec![0, 0],
        )),
        expected_sort(expected_sort_vec(vec![[0, 1], [1, 0], [3, 3]]))
    );
    assert_eq!(
        expected_sort(Solution::queens_attackthe_king_v1(
            expected_sort_vec(vec![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],),
            vec![3, 3],
        )),
        expected_sort(expected_sort_vec(vec![[2, 2], [3, 4], [4, 4]]))
    );
    assert_eq!(
        expected_sort(Solution::queens_attackthe_king_v1(
            expected_sort_vec(vec![
                [5, 6],
                [7, 7],
                [2, 1],
                [0, 7],
                [1, 6],
                [5, 1],
                [3, 7],
                [0, 3],
                [4, 0],
                [1, 2],
                [6, 3],
                [5, 0],
                [0, 4],
                [2, 2],
                [1, 1],
                [6, 4],
                [5, 4],
                [0, 0],
                [2, 6],
                [4, 5],
                [5, 2],
                [1, 4],
                [7, 5],
                [2, 3],
                [0, 5],
                [4, 2],
                [1, 0],
                [2, 7],
                [0, 1],
                [4, 6],
                [6, 1],
                [0, 6],
                [4, 3],
                [1, 7]
            ],),
            vec![3, 4],
        )),
        expected_sort(expected_sort_vec(vec![
            [2, 3],
            [1, 4],
            [1, 6],
            [3, 7],
            [4, 3],
            [5, 4],
            [4, 5]
        ]))
    );
}