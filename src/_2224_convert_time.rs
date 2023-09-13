// https://leetcode.cn/problems/minimum-number-of-operations-to-convert-time

#![allow(unused)]
use crate::types::base_type::Solution;

impl Solution {
    /// 转化时间需要的最少操作数
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current: Vec<_> = current.split(':').collect();
        let correct: Vec<_> = correct.split(':').collect();
        let mut current =
            current[0].parse::<i32>().unwrap() * 60 + current[1].parse::<i32>().unwrap();
        let mut correct =
            correct[0].parse::<i32>().unwrap() * 60 + correct[1].parse::<i32>().unwrap();

        let mut res = 0;
        while correct != current {
            for i in &[60, 15, 5, 1] {
                if current + *i <= correct {
                    current += *i;
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}
