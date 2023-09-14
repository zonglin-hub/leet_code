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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman_v1() {
        /*
            输入: num = 3
            输出: "III"
        */
        assert_eq!(Solution::int_to_roman_v1(3), "III");

        /*
            输入: num = 4
            输出: "IV"
        */
        assert_eq!(Solution::int_to_roman_v1(4), "IV");

        /*
            输入: num = 9
            输出: "IX"
        */
        assert_eq!(Solution::int_to_roman_v1(9), "IX");

        /*
            输入: num = 58
            输出: "LVIII"
            解释: L = 50, V = 5, III = 3.
        */
        assert_eq!(Solution::int_to_roman_v1(58), "LVIII");

        /*
            输入: num = 1994
            输出: "MCMXCIV"
            解释: M = 1000, CM = 900, XC = 90, IV = 4.
        */
        assert_eq!(Solution::int_to_roman_v1(1994), "MCMXCIV");
    }
}
