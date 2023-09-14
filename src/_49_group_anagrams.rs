//! 字母异位词分组

#![allow(unused)]

use crate::types::base_type::Solution;
use std::collections::HashMap;

impl Solution {
    /// `group_anagrams()` 函数用于将一个字符串数组按照由相同字符组成的异位词分组，并返回一个二维字符串数组，其中每个子数组包含相同字符组成的异位词集合。
    /// 该函数使用了一种基于质数乘积的哈希算法，将每个字符串中的字符转换成对应的质数，然后将所有质数相乘得到一个哈希值。
    /// 由于质数乘积是唯一的，因此相同字符组成的异位词得到的哈希值一定相同。
    /// 对字符串数组进行fold操作，对于每个字符串，计算其哈希值，并在哈希表中查找对应的字符串集合，如果已经存在，则将该字符串添加到集合中；否则，在哈希表中插入新的键值对。
    /// 最后使用drain()方法将哈希表转换为迭代器，然后对于每个键值对，取出其值部分得到一个字符串集合，最终使用collect()方法将字符串集合收集到一个二维字符串数组中并返回。
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 常量数组PRIME，用于保存前26个质数
        const PRIME: [u128; 26] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        // 使用一个空哈希表hash_map保存字符串的哈希值和相应的字符串集合
        let hash_map = HashMap::<_, Vec<String>>::new();

        strs.into_iter() // 将输入字符串数组strs转换为迭代器。
            // 对于迭代器中的每个元素，使用fold方法将其聚合到哈希表中。fold方法接受两个参数：初始累加器和迭代器处理函数。
            // 在这里，初始累加器是空的哈希表hash_map，迭代器处理函数是一个闭包，接受两个参数：一个可变的哈希表map和一个字符串f。
            .fold(hash_map, |mut map, f| {
                // 计算字符串的哈希值，将其作为键，查找哈希表中对应的值。
                map.entry(
                    f.chars()
                        .fold(1, |a, b| a * PRIME[(b as usize) - ('a' as usize)]),
                )
                // 如果该键已经存在，则将输入字符串f添加到对应的字符串集合中；
                // 否则，在哈希表中插入一个新的键值对，键为哈希值，值为一个包含输入字符串f的单元素字符串集合。
                .and_modify(|v| v.push(f.clone()))
                .or_insert_with(|| vec![f.clone()]);
                // 迭代器处理完毕后返回聚合结果，即一个包含所有字符串集合的哈希表。
                map
            })
            // 将哈希表转换为迭代器，同时删除哈希表中的所有键值对。
            .drain()
            // 将迭代器中每个键值对的值部分取出，即一个字符串集合。
            .map(|(_, v)| v)
            // 将所有字符串集合收集到一个二维字符串数组中，并作为函数的返回值。
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        // 将每个字母异位词列表内的字符串排序，确保输出顺序与期望输出顺序一致
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
}
