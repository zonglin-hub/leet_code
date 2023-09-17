use leet_code::leet_code::Solution;

#[test]
fn test_find_substring() {
    assert_eq!(
        vec![0, 9],
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        )
    );

    assert_eq!(
        Vec::<i32>::new(),
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string(),
            ]
            .to_vec()
        )
    );
}
