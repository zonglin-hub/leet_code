//! 去掉最低工资和最高工资后的工资平均值

use crate::types::base_type::Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>()
            - salary.iter().min().expect("")
            - salary.iter().max().expect("")) as f64
            / (salary.len() - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        /*
            输入：salary = [4000,3000,1000,2000]
            输出：2500.00000
            解释：最低工资和最高工资分别是 1000 和 4000 。
            去掉最低工资和最高工资以后的平均工资是 (2000+3000)/2= 2500
        */
        assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);

        /*
            输入：salary = [1000,2000,3000]
            输出：2000.00000
            解释：最低工资和最高工资分别是 1000 和 3000 。
            去掉最低工资和最高工资以后的平均工资是 (2000)/1= 2000
        */
        assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);

        /*
            输入：salary = [6000,5000,4000,3000,2000,1000]
            输出：3500.00000
        */
        assert_eq!(
            Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
            3500.00000
        );

        /*
            输入：salary = [8000,9000,2000,3000,6000,1000]
            输出：4750.00000
        */
        assert_eq!(
            Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
            4750.00000
        );
    }
}
