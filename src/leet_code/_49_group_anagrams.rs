//! 字母异位词分组

// 常量数组PRIME，用于保存前26个质数
const PRIME: [u128; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101,
];

use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::<_, Vec<String>>::new(), |mut map, f| {
                map.entry(
                    f.chars()
                        .fold(1, |a, b| a * PRIME[(b as usize) - ('a' as usize)]),
                )
                .and_modify(|v| v.push(f.clone()))
                .or_insert_with(|| vec![f.clone()]);
                map
            })
            .drain()
            .map(|(_, v)| v)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

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
}
