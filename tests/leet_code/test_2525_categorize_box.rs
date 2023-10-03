use leet_code::leet_code::Solution;

#[test]
fn test_categorize_box() {
    assert_eq!(
        Solution::categorize_box(1000, 35, 700, 300),
        "Heavy".to_string()
    );
    assert_eq!(
        Solution::categorize_box(200, 50, 800, 50),
        "Neither".to_string()
    );
}

#[test]
fn test_categorize_box_() {
    assert_eq!(
        Solution::categorize_box_(1000, 35, 700, 300),
        "Heavy".to_string()
    );
    assert_eq!(
        Solution::categorize_box_(200, 50, 800, 50),
        "Neither".to_string()
    );
}
