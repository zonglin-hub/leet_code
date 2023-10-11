//! 最小和分割

use super::Solution;

impl Solution {
    /// 贪心+排序(for)
    pub fn split_num(num: i32) -> i32 {
        let mut s = num.to_string().bytes().collect::<Vec<_>>();
        s.sort();
        let mut a = [0; 2];

        for (i, &c) in s.iter().enumerate() {
            let idx = i % 2;
            a[idx] = a[idx] * 10 + c as i32 - '0' as i32
        }

        a[0] + a[1]
    }

    /// 贪心+排序(fold)
    pub fn split_num_v1(num: i32) -> i32 {
        let mut nums = num.to_string().bytes().collect::<Vec<_>>();
        nums.sort();

        let a = nums.iter().enumerate().fold([0; 2], |mut acc, (i, &c)| {
            let idx = i % 2;
            acc[idx] = acc[idx] * 10 + (c as i32 - '0' as i32);
            acc
        });

        a[0] + a[1]
    }

    /// (0, 0)
    pub fn split_num_v2(num: i32) -> i32 {
        let mut nums = num
            .to_string()
            .chars()
            .map(|c| c as i32 - 48)
            .collect::<Vec<_>>();
        nums.sort();

        let num = nums.iter().fold((0, 0), |(mut a, b), ipt| {
            a = a * 10 + ipt;
            // (a, b) 不可交互值
            (b, a)
        });
        num.0 + num.1
    }
}
