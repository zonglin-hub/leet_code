use leet_code::leet_code::Solution;

#[test]
fn test_find_substring() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    assert_eq!(vec![0, 9], Solution::find_substring(s, words));

    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec![
        "word".to_string(),
        "good".to_string(),
        "best".to_string(),
        "word".to_string(),
    ]
    .to_vec();
    assert_eq!(Vec::<i32>::new(), Solution::find_substring(s, words));
}
