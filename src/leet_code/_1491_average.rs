//! 去掉最低工资和最高工资后的工资平均值

use super::Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>() - salary.iter().min().unwrap() - salary.iter().max().unwrap())
            as f64
            / (salary.len() - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_average() {
        assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);

        assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);

        assert_eq!(
            Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
            3500.00000
        );

        assert_eq!(
            Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
            4750.00000
        );
    }
}
