use leet_code::leet_code::Solution;

#[test]
fn test_count_and_say() {
    assert_eq!(Solution::count_and_say(1), "1".to_string());
    assert_eq!(Solution::count_and_say(4), "1211".to_string());
}
