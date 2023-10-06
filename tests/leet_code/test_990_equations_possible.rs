use leet_code::leet_code::Solution;

#[test]
fn test_equations_possible() {
    assert_eq!(
        Solution::equations_possible(vec!["b==a".to_string(), "a==b".to_string()]),
        true
    );
    assert_eq!(
        Solution::equations_possible(vec![
            "a==b".to_string(),
            "b==c".to_string(),
            "a==c".to_string()
        ]),
        true
    );

    assert_eq!(
        Solution::equations_possible(vec![
            "a==b".to_string(),
            "b!=c".to_string(),
            "c==a".to_string()
        ]),
        false
    );

    assert_eq!(
        Solution::equations_possible(vec![
            "c==c".to_string(),
            "b==d".to_string(),
            "x!=z".to_string()
        ]),
        true
    );
}

#[test]
fn test_partition() {
    let a = [1, 2, 3];

    let (even, odd): (Vec<_>, Vec<_>) = a.into_iter().partition(|n| n % 2 == 0);

    assert_eq!(even, vec![2]);
    assert_eq!(odd, vec![1, 3]);
}
