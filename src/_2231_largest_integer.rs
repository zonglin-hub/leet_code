//! 按奇偶性交换后的最大数字
//!

use crate::Solution;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_integer() {
        /*
            输入：num = 1234
            输出：3412
            解释：交换数字 3 和数字 1 ，结果得到 3214 。
            交换数字 2 和数字 4 ，结果得到 3412 。
            注意，可能存在其他交换序列，但是可以证明 3412 是最大可能值。
            注意，不能交换数字 4 和数字 1 ，因为它们奇偶性不同。
        */
        assert_eq!(Solution::largest_integer(1234), 3412);

        /*
            输入：num = 65875
            输出：87655
            解释：交换数字 8 和数字 6 ，结果得到 85675 。
            交换数字 5 和数字 7 ，结果得到 87655 。
            注意，可能存在其他交换序列，但是可以证明 87655 是最大可能值。
        */
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}
