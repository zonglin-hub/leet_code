use super::Solution;

impl Solution {
    pub fn max_profit_122(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|x| x[1] - x[0])
            .filter(|&x| x > 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_profit_122() {
        assert_eq!(Solution::max_profit_122(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit_122(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit_122(vec![7, 6, 4, 3, 1]), 0);
    }
}
