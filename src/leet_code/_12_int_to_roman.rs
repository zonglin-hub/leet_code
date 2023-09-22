//! 整数转罗马数字
//! 相似题型：12 | 13

use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn int_to_roman_12_v1(num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let rn = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut ans = Vec::new();
        let mut i = 0;
        let mut num = num;
        while i < values.len() && num >= 0 {
            while values[i] <= num {
                num -= values[i];
                ans.push(rn[i])
            }
            i += 1;
        }
        ans.into_iter().collect()
    }
}

impl Solution {
    pub fn roman_to_int_13_v1(s: String) -> i32 {
        let dic = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let chars = s.chars().collect::<Vec<char>>();
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
