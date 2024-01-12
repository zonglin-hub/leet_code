//! 转化时间需要的最少操作数

use super::Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current = current.split(':').collect::<Vec<_>>();
        let correct = correct.split(':').collect::<Vec<_>>();
        let mut current =
            current[0].parse::<i32>().unwrap() * 60 + current[1].parse::<i32>().unwrap();
        let correct = correct[0].parse::<i32>().unwrap() * 60 + correct[1].parse::<i32>().unwrap();
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
    use crate::leet_code::Solution;

    #[test]
    fn test_convert_time() {
        assert_eq!(Solution::convert_time("02:30".to_string(), "04:35".to_string()), 3);
        assert_eq!(Solution::convert_time("11:00".to_string(), "11:01".to_string()), 1);
    }
}
