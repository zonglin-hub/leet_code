//! 罗马数字转整数

use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn roman_to_int_v1(s: String) -> i32 {
        let dic = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let chars: Vec<char> = s.chars().collect();
        let mut ans = *dic.get(&chars[chars.len() - 1]).expect("");
        for c in 0..chars.len() - 1 {
            let current_unit = dic.get(&chars[c]).expect("");
            let next_unit = dic.get(&chars[c + 1]).expect("");
            if current_unit >= next_unit {
                ans += current_unit
            } else {
                ans -= current_unit
            }
        }
        ans
    }
}
