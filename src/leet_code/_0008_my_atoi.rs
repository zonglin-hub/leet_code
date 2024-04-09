//! 字符串转换整数 (atoi)

use super::Solution;

impl Solution {
    pub fn my_atoi_v1(str: String) -> i32 {
        let mut negative = false;
        let mut res = 0_i64;
        for (i, ch) in str.trim().chars().enumerate() {
            if ch == '+' && i == 0 {
                continue;
            }
            if ch == '-' && i == 0 {
                negative = true;
                continue;
            }
            if !ch.is_ascii_digit() {
                break;
            }
            res = 10_i64 * res + ch.to_digit(10).unwrap() as i64;
            match negative {
                true => {
                    if -res < i32::MIN as i64 {
                        return i32::MIN;
                    }
                }
                false => {
                    if res > i32::MAX as i64 {
                        return i32::MAX;
                    }
                }
            }
        }
        match negative {
            true => -res as i32,
            false => res as i32,
        }
    }
}

enum Status {
    Waiting,             // 等等..
    PositiveNumber(i32), // 正数
    NegativeNumber(i32), // 负数
    End(i32),            // 结束
}

impl Solution {
    /// 自动机 (力扣官方题解)
    pub fn my_atoi(s: String) -> i32 {
        fn overflowing_muli(a: i32) -> (i32, bool) {
            match a.overflowing_mul(10) {
                (_, true) => match a > 0 {
                    true => (i32::MAX, true),
                    false => (i32::MIN, true),
                },
                (ans, false) => (ans, false),
            }
        }

        fn overflowing_muli_add(a: i32, b: i32) -> (i32, bool) {
            match overflowing_muli(a) {
                (o, true) => (o, true),
                (a, false) => match a.overflowing_add(b) {
                    (_, true) => (i32::MAX, true),
                    (ans, false) => (ans, false),
                },
            }
        }

        fn overflowing_muli_sub(a: i32, b: i32) -> (i32, bool) {
            match overflowing_muli(a) {
                (o, true) => (o, true),
                (a, false) => match a.overflowing_sub(b) {
                    (_, true) => (i32::MIN, true),
                    (ans, false) => (ans, false),
                },
            }
        }

        let mut status = Status::Waiting;
        let to_int = |ch: char| (ch as u8 - b'0') as i32;

        for ch in s.chars() {
            status = match status {
                Status::Waiting => match ch {
                    ' ' => continue,
                    '-' => Status::NegativeNumber(0),
                    '+' => Status::PositiveNumber(0),
                    '0'..='9' => Status::PositiveNumber(to_int(ch)),
                    _ => Status::End(0),
                },
                Status::PositiveNumber(a) => match ch {
                    '0'..='9' => match overflowing_muli_add(a, to_int(ch)) {
                        (o, true) => Status::End(o),
                        (ans, false) => Status::PositiveNumber(ans),
                    },
                    _ => Status::End(a),
                },
                Status::NegativeNumber(a) => match ch {
                    '0'..='9' => match overflowing_muli_sub(a, to_int(ch)) {
                        (o, true) => Status::End(o),
                        (ans, false) => Status::NegativeNumber(ans),
                    },
                    _ => Status::End(a),
                },
                Status::End(_) => break,
            };
        }

        match status {
            Status::PositiveNumber(ans) | Status::NegativeNumber(ans) | Status::End(ans) => ans,
            Status::Waiting => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    fn test_my_atoi_v1() {
        assert_eq!(Solution::my_atoi_v1(String::from("42")), 42);
        assert_eq!(Solution::my_atoi_v1(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi_v1(String::from("4193 with words")), 4193);
    }
}
