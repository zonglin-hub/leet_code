#![allow(unused)]

use std::collections::HashMap;
use crate::types::base_type::Solution;

impl Solution {
    /// 去掉最低工资和最高工资后的工资平均值
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>() - salary.iter().min().unwrap() - salary.iter().max().unwrap())
            as f64
            / (salary.len() - 2) as f64
    }
}
