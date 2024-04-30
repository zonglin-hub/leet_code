use super::Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        for i in hours {
            if i >= target {
                ans += 1
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2), 3);
        assert_eq!(Solution::number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6), 0);
    }
}
