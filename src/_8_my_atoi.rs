//! 字符串转换整数 (atoi)

use crate::Solution;

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
