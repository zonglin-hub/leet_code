use leet_code::leet_code::Solution;

#[test]
fn test_minimum_buckets() {
    assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);
    assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);
    assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);
    assert_eq!(Solution::minimum_buckets("H".to_string()), -1);
    assert_eq!(Solution::minimum_buckets(".".to_string()), 0);
}
