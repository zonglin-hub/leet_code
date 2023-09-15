use leet_code::Solution;

#[test]
fn test_group_anagrams() {
    let strs = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    let expected_output = vec![
        vec![String::from("bat")],
        vec![String::from("nat"), String::from("tan")],
        vec![
            String::from("ate"),
            String::from("eat"),
            String::from("tea"),
        ],
    ];
    let actual_output = Solution::group_anagrams(strs);
    let mut sorted_actual = actual_output
        .into_iter()
        .map(|mut group| {
            group.sort();
            group
        })
        .collect::<Vec<Vec<String>>>();
    sorted_actual.sort();
    let mut sorted_expected = expected_output
        .into_iter()
        .map(|mut group| {
            group.sort();
            group
        })
        .collect::<Vec<Vec<String>>>();
    sorted_expected.sort();
    assert_eq!(sorted_actual, sorted_expected);
}
