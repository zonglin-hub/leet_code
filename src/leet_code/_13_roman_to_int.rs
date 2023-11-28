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
        let chars = s.chars().collect::<Vec<char>>();
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
    use crate::leet_code::Solution;

    #[test]
    fn test_roman_to_int_v1() {
        assert_eq!(Solution::roman_to_int_v1("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int_v1("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int_v1("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int_v1("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int_v1("MCMXCIV".to_string()), 1994);
    }
}
