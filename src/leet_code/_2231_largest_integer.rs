//! 按奇偶性交换后的最大数字
//!

use super::Solution;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        // 将num转换为字符串
        let mut s = num.to_string().chars().collect::<Vec<char>>();
        // 获取字符串的长度
        let n = s.len();

        // 遍历字符串，从第一个字符开始，比较每个字符的值，如果比较的值小于等于第二个字符，则交换位置
        for i in 0..n - 1 {
            for j in i + 1..n {
                if s[i] < s[j] && s[i] as u8 % 2 == s[j] as u8 % 2 {
                    s.swap(i, j);
                }
            }
        }
        // 将字符串转换为字符串，并转换为i32类型
        s.into_iter().collect::<String>().parse::<i32>().expect("")
    }
}
