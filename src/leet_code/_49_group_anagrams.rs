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
            // 对于迭代器中的每个元素，使用fold方法将其聚合到哈希表中。fold方法接受两个参数：初始累加器和迭代器处理函数。
            // 在这里，初始累加器是空的哈希表hash_map，迭代器处理函数是一个闭包，接受两个参数：一个可变的哈希表map和一个字符串f。
            .fold(HashMap::<_, Vec<String>>::new(), |mut map, f| {
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
