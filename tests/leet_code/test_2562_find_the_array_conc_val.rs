use leet_code::leet_code::Solution;

#[test]
fn test_find_the_array_conc_val() {
    assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
    assert_eq!(
        Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
        673
    );
}
