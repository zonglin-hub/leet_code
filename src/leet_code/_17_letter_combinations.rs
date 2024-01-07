//! 电话号码的字母组合

use std::collections::HashMap;

use super::Solution;

/// error: associated `static` items are not allowed
/// 错误是因为 static 变量声明必须在 impl 块之外，而你的代码将它放在了 impl Solution 块里。
const DATA: &[(&str, &[char])] = &[
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
        #[inline]
        fn dfs(idx: usize, digits: &str, path: &mut String, ans: &mut Vec<String>) {
            if digits.len() == idx {
                return ans.push(path.clone());
            }

            for &ch in DATA
                .iter()
                .find(|&&(d, _)| d == digits.chars().nth(idx).unwrap().to_string().as_str())
                .map(|&(_, cs)| cs)
                .unwrap_or(&[])
            {
                path.push(ch);
                dfs(idx + 1, digits, path, ans);
                path.pop();
            }
        }

        let mut ans = Vec::new();
        if digits.is_empty() {
            return ans;
        }

        dfs(
            0,
            &digits,
            &mut String::with_capacity(digits.len()),
            &mut ans,
        );

        ans
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        #[inline]
        fn dfs(
            idx: usize,
            digits: &String,
            map: &HashMap<char, String>,
            path: &mut String,
            ans: &mut Vec<String>,
        ) {
            if digits.len() == idx {
                return ans.push(path.clone());
            }

            for ch in map.get(&digits.chars().nth(idx).unwrap()).unwrap().chars() {
                path.push(ch);
                dfs(idx + 1, digits, map, path, ans);
                path.pop();
            }
        }

        let mut ans = Vec::new();
        if digits.is_empty() {
            return ans;
        }

        dfs(
            0,
            &digits,
            &HashMap::from([
                ('2', "abc".to_string()),
                ('3', "def".to_string()),
                ('4', "ghi".to_string()),
                ('5', "jkl".to_string()),
                ('6', "mno".to_string()),
                ('7', "pqrs".to_string()),
                ('8', "tuv".to_string()),
                ('9', "wxyz".to_string()),
            ]),
            &mut String::new(),
            &mut ans,
        );

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_letter_combinations_v1() {
        assert_eq!(
            Solution::letter_combinations_v1("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations_v1("".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations_v1("2".to_string()),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
