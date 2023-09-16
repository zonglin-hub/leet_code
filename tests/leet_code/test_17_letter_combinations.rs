use leet_code::leet_code::Solution;

#[test]
fn test_letter_combinations_v1() {
    assert_eq!(
        Solution::letter_combinations_v1("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(
        Solution::letter_combinations_v1("".to_string()),
        Vec::<String>::new()
    );
    assert_eq!(
        Solution::letter_combinations_v1("2".to_string()),
        vec!["a", "b", "c"]
    );
}

#[test]
fn test_letter_combinations_v2() {
    assert_eq!(
        Solution::letter_combinations_v2("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(
        Solution::letter_combinations_v2("".to_string()),
        Vec::<String>::new()
    );
    assert_eq!(
        Solution::letter_combinations_v2("2".to_string()),
        vec!["a", "b", "c"]
    );
}
