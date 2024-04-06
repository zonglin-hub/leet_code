use super::Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let step = 2;
        let begin = 1 - n;

        (begin..n).step_by(step).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        // assert_eq!(Solution::sum_zero(5), vec![-7, -1, 1, 3, 4]);
        // assert_eq!(Solution::sum_zero(3), vec![-1, 0, 1]);
        assert_eq!(Solution::sum_zero(1), vec![0]);
    }
}
