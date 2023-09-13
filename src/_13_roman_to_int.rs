//! 罗马数字转整数
//!
//! 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。

use std::collections::HashMap;

use crate::types::base_type::Solution;

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
        let mut ans = *dic.get(&chars[chars.len() - 1]).unwrap();
        for c in 0..chars.len() - 1 {
            let current_unit = dic.get(&chars[c]).unwrap();
            let next_unit = dic.get(&chars[c + 1]).unwrap();
            if current_unit >= next_unit {
                ans += current_unit
            } else {
                ans -= current_unit
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int_v1() {
        /*
            输入: s = "III"
            输出: 3
        */
        assert_eq!(Solution::roman_to_int_v1("III".to_string()), 3);

        /*
            输入: s = "IV"
            输出: 4
        */
        assert_eq!(Solution::roman_to_int_v1("IV".to_string()), 4);

        /*
            输入: s = "IX"
            输出: 9
        */
        assert_eq!(Solution::roman_to_int_v1("IX".to_string()), 9);

        /*
            输入: s = "LVIII"
            输出: 58
            解释: L = 50, V= 5, III = 3.
        */
        assert_eq!(Solution::roman_to_int_v1("LVIII".to_string()), 58);

        /*
            输入: s = "MCMXCIV"
            输出: 1994
            解释: M = 1000, CM = 900, XC = 90, IV = 4.
        */
        assert_eq!(Solution::roman_to_int_v1("MCMXCIV".to_string()), 1994);
    }
}
