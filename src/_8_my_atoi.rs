//! 字符串转换整数 (atoi)

use crate::types::base_type::Solution;

impl Solution {
    pub fn my_atoi_v1(s: String) -> i32 {
        let s = s.trim_start();
        let mut num = 0;
        let mut sign = 1;
        for (i, b) in s.bytes().enumerate() {
            if i == 0 {
                sign = match b {
                    b'+' => 1,
                    b'-' => -1,
                    b'0'..=b'9' => {
                        num = (b - b'0') as i32;
                        1
                    }
                    _ => return 0,
                };
            } else {
                match b {
                    b'0'..=b'9' => {
                        num = match num.checked_mul(10) {
                            Some(val) => val,
                            None => match sign == 1 {
                                true => return i32::MAX,
                                false => return i32::MIN,
                            },
                        };
                        let v = (b - b'0') as i32 * sign;
                        num = match num.checked_add(v) {
                            Some(val) => val,
                            None => match sign == 1 {
                                true => return i32::MAX,
                                false => return i32::MIN,
                            },
                        };
                    }
                    _ => break,
                }
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi_v1() {
        /*
            输入：s = "42"
            输出：42
            解释：加粗的字符串为已经读入的字符，插入符号是当前读取的字符。
            第 1 步："42"（当前没有读入字符，因为没有前导空格）
                    ^
            第 2 步："42"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
                    ^
            第 3 步："42"（读入 "42"）
                    ^
            解析得到整数 42 。
            由于 "42" 在范围 [-231, 231 - 1] 内，最终结果为 42 。
        */
        assert_eq!(Solution::my_atoi_v1(String::from("42")), 42);

        /*
            输入：s = "   -42"
            输出：-42
            解释：
            第 1 步："   -42"（读入前导空格，但忽视掉）
                        ^
            第 2 步："   -42"（读入 '-' 字符，所以结果应该是负数）
                        ^
            第 3 步："   -42"（读入 "42"）
                        ^
            解析得到整数 -42 。
            由于 "-42" 在范围 [-231, 231 - 1] 内，最终结果为 -42 。
        */
        assert_eq!(Solution::my_atoi_v1(String::from("   -42")), -42);

        /*
            输入：s = "4193 with words"
            输出：4193
            解释：
            第 1 步："4193 with words"（当前没有读入字符，因为没有前导空格）
                    ^
            第 2 步："4193 with words"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
                    ^
            第 3 步："4193 with words"（读入 "4193"；由于下一个字符不是一个数字，所以读入停止）
                        ^
            解析得到整数 4193 。
            由于 "4193" 在范围 [-231, 231 - 1] 内，最终结果为 4193 。
        */
        assert_eq!(Solution::my_atoi_v1(String::from("4193 with words")), 4193);
    }
}
