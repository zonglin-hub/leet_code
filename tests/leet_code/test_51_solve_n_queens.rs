use leet_code::leet_code::Solution;

#[test]
fn test_my_pow() {
    assert_eq!(
        Solution::solve_n_queens(4),
        vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."]
        ]
    );
    assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
}

#[test]
fn test_vec() {
    let a = [1, 3, 4, 2];
    let n = a.len();
    for (i, _) in a.iter().enumerate().take(n) {
        println!("ele: {}", i);
    }

    for i in a.iter().take(n) {
        println!("ele: {}", i);
    }

    for i in 0..n {
        println!("{}", i);
    }
}
