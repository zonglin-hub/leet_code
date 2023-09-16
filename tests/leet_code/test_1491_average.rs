use leet_code::leet_code::Solution;

#[test]
fn test_average() {
    assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);

    assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);

    assert_eq!(
        Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
        3500.00000
    );

    assert_eq!(
        Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
        4750.00000
    );
}
