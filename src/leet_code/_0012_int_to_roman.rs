//! 整数转罗马数字

use super::Solution;

const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"]; // 1000~3000
const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]; // 100~900
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]; // 10~90
const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]; // 1~9

impl Solution {
    /// 硬编码数字 (力扣官方题解)
    pub fn int_to_roman(n: usize) -> String {
        format!(
            "{}{}{}{}",
            THOUSANDS[n / 1000],
            HUNDREDS[(n % 1000) / 100],
            TENS[(n % 100) / 10],
            ONES[n % 10]
        )
    }
}

impl Solution {
    /// 模拟 (力扣官方题解)
    pub fn int_to_roman_v1(mut num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut ans = String::new();
        let mut i = 0;
        while i < values.len() && num >= 0 {
            while values[i] <= num {
                num -= values[i];
                ans.push_str(symbols[i])
            }
            i += 1;
        }
        ans
    }
}

const MAP: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Solution {
    // 模拟
    pub fn int_to_roman_v2(mut num: i32) -> String {
        let mut ans = String::new();
        for (k, v) in MAP {
            while num >= k {
                ans.push_str(v);
                num -= k;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test_int_to_roman_v1() {
        assert_eq!(Solution::int_to_roman_v1(3), "III");
        assert_eq!(Solution::int_to_roman_v1(4), "IV");
        assert_eq!(Solution::int_to_roman_v1(9), "IX");
        assert_eq!(Solution::int_to_roman_v1(58), "LVIII");
        assert_eq!(Solution::int_to_roman_v1(1994), "MCMXCIV");
    }

    #[test]
    fn test_int_to_roman_v2() {
        assert_eq!(Solution::int_to_roman_v2(3), "III");
        assert_eq!(Solution::int_to_roman_v2(4), "IV");
        assert_eq!(Solution::int_to_roman_v2(9), "IX");
        assert_eq!(Solution::int_to_roman_v2(58), "LVIII");
        assert_eq!(Solution::int_to_roman_v2(1994), "MCMXCIV");
    }
}
