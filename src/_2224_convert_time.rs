//! 转化时间需要的最少操作数

use crate::types::base_type::Solution;

impl Solution {
    /// 转化时间需要的最少操作数
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current: Vec<_> = current.split(':').collect();
        let correct: Vec<_> = correct.split(':').collect();
        let mut current =
            current[0].parse::<i32>().expect("") * 60 + current[1].parse::<i32>().expect("");
        let correct =
            correct[0].parse::<i32>().expect("") * 60 + correct[1].parse::<i32>().expect("");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_time() {
        /*
            输入：current = "02:30", correct = "04:35"
            输出：3
            解释：
            可以按下述 3 步操作将 current 转换为 correct ：
            - 为 current 加 60 分钟，current 变为 "03:30" 。
            - 为 current 加 60 分钟，current 变为 "04:30" 。
            - 为 current 加 5 分钟，current 变为 "04:35" 。
            可以证明，无法用少于 3 步操作将 current 转化为 correct 。
        */
        assert_eq!(
            Solution::convert_time("02:30".to_string(), "04:35".to_string()),
            3
        );

        /*
            输入：current = "11:00", correct = "11:01"
            输出：1
            解释：只需要为 current 加一分钟，所以最小操作数是 1 。
        */
        assert_eq!(
            Solution::convert_time("11:00".to_string(), "11:01".to_string()),
            1
        );
    }
}
