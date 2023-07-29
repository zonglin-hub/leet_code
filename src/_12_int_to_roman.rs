#![allow(unused)]

struct Solution;

impl Solution {
    /// 整数转罗马数字
    pub fn int_to_roman(num: i32) -> String {
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
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
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
