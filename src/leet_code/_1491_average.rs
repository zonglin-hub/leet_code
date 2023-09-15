//! 去掉最低工资和最高工资后的工资平均值

use crate::Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>()
            - salary.iter().min().expect("")
            - salary.iter().max().expect("")) as f64
            / (salary.len() - 2) as f64
    }
}
