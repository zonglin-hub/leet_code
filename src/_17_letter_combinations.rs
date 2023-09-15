//! 电话号码的字母组合

use std::collections::HashMap;

use crate::Solution;

/// error: associated `static` items are not allowed
/// 错误是因为 static 变量声明必须在 impl 块之外，而你的代码将它放在了 impl Solution 块里。

const MAP: &[(&str, &[char])] = &[
    ("2", &['a', 'b', 'c']),
    ("3", &['d', 'e', 'f']),
    ("4", &['g', 'h', 'i']),
    ("5", &['j', 'k', 'l']),
    ("6", &['m', 'n', 'o']),
    ("7", &['p', 'q', 'r', 's']),
    ("8", &['t', 'u', 'v']),
    ("9", &['w', 'x', 'y', 'z']),
];

impl Solution {
    /// 将 map 定义为全局变量，避免在每次递归调用时都重新创建。
    /// 将 String::new() 改为 String::with_capacity(digits.len())，避免在递归调用时频繁分配内存。
    /// 在 dfs 函数中将 digits 参数改为 &str 类型，避免产生不必要的 String 对象。
    pub fn letter_combinations_v1(digits: String) -> Vec<String> {
        let mut ans = Vec::new();
        if digits.is_empty() {
            return ans;
        }
        Self::dfs_v1(
            0,
            &digits,
            &mut String::with_capacity(digits.len()),
            &mut ans,
        );
        ans
    }

    fn dfs_v1(idx: usize, digits: &str, path: &mut String, ans: &mut Vec<String>) {
        if digits.len() == idx {
            ans.push(path.clone());
            return;
        }
        if let Some(c) = digits.chars().nth(idx) {
            for &ch in MAP
                .iter()
                .find(|&&(d, _)| d == c.to_string().as_str())
                .map(|&(_, cs)| cs)
                .unwrap_or(&[])
            {
                path.push(ch);
                Self::dfs_v1(idx + 1, digits, path, ans);
                path.pop();
            }
        }
    }
}

impl Solution {
    pub fn letter_combinations_v2(digits: String) -> Vec<String> {
        let map = std::collections::HashMap::from([
            ('2', "abc".to_string()),
            ('3', "def".to_string()),
            ('4', "ghi".to_string()),
            ('5', "jkl".to_string()),
            ('6', "mno".to_string()),
            ('7', "pqrs".to_string()),
            ('8', "tuv".to_string()),
            ('9', "wxyz".to_string()),
        ]);

        let mut ans = Vec::new();
        if digits.is_empty() {
            return ans;
        }
        Self::dfs_v2(0, &digits, &map, &mut String::new(), &mut ans);
        ans
    }

    fn dfs_v2(
        idx: usize,
        digits: &String,
        map: &HashMap<char, String>,
        path: &mut String,
        ans: &mut Vec<String>,
    ) {
        if digits.len() == idx {
            ans.push(path.clone());
            return;
        }
        let cc = digits.chars().nth(idx).expect("");
        for c in map.get(&cc).expect("").chars() {
            path.push(c);
            Self::dfs_v2(idx + 1, digits, map, path, ans);
            path.pop();
        }
    }
}
