#![allow(unused)]
struct Solution;

impl Solution {
    /// 罗马数字转整数
    pub fn roman_to_int(s: String) -> i32 {
        let dic = std::collections::HashMap::from([
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
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
