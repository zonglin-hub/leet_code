use leet_code::leet_code::Solution;

#[test]
fn test_group_anagrams() {
    let mut sorted_actual = Solution::group_anagrams(vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ])
    .into_iter()
    .map(|mut group| {
        group.sort();
        group
    })
    .collect::<Vec<Vec<String>>>();
    sorted_actual.sort();

    let mut sorted_expected = vec![
        vec![String::from("bat")],
        vec![String::from("nat"), String::from("tan")],
        vec![
            String::from("ate"),
            String::from("eat"),
            String::from("tea"),
        ],
    ]
    .into_iter()
    .map(|mut group| {
        group.sort();
        group
    })
    .collect::<Vec<Vec<String>>>();
    sorted_expected.sort();

    assert_eq!(sorted_actual, sorted_expected);
}
