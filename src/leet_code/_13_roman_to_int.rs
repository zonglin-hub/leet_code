//! 罗马数字转整数

use std::collections::HashMap;

use super::Solution;

const VALUES: [(char, i32); 7] = [
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];

impl Solution {
    /// 模拟 (力扣官方题解)
    pub fn roman_to_int(s: String) -> i32 {
        let symbol_values = HashMap::from(VALUES);
        let ch = s.chars().collect::<Vec<char>>();
        let mut ans = *symbol_values.get(&ch[ch.len() - 1]).unwrap();
        (0..ch.len() - 1).for_each(|i| {
            let current = symbol_values.get(&ch[i]).unwrap();
            if current >= symbol_values.get(&ch[i + 1]).unwrap() {
                ans += current
            } else {
                ans -= current
            }
        });
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
