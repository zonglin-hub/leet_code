//! 买卖股票的最佳时机。

use super::Solution;

impl Solution {
    pub fn max_profit_121(prices: Vec<i32>) -> i32 {
        let (mut max_profit, mut min_price) = (0, std::i32::MAX);
        for price in prices {
            min_price = std::cmp::min(min_price, price);
            max_profit = std::cmp::max(max_profit, price - min_price);
        }
        max_profit
    }

    pub fn max_profit_121_v2(prices: Vec<i32>) -> i32 {
        let (mut res, mut min) = (0, prices[0]);
        for i in prices {
            match i - min {
                t if t > res => res = t,
                t if t < 0 => min = i,
                _ => (),
            }
        }
        res
    }

    pub fn max_profit_121_v3(prices: Vec<i32>) -> i32 {
        let (mut res, mut min) = (0, i32::MAX);
        for i in prices.iter() {
            if *i < min {
                min = *i;
            }
            if i - min > res {
                res = i - min;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_profit_121() {
        assert_eq!(Solution::max_profit_121(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_121(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_121_v2() {
        assert_eq!(Solution::max_profit_121_v2(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_121_v2(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_121_v3() {
        assert_eq!(Solution::max_profit_121_v3(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_121_v3(vec![7, 6, 4, 3, 1]), 0);
    }
}
