//! 整数转罗马数字
//!
use crate::Solution;

impl Solution {
    pub fn int_to_roman_v1(num: i32) -> String {
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
