//! 买卖股票的最佳时机

use crate::Solution;

impl Solution {
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let (mut max_profit, mut min_price) = (0, std::i32::MAX);
        for price in prices {
            min_price = std::cmp::min(min_price, price);
            max_profit = std::cmp::max(max_profit, price - min_price);
        }
        max_profit
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
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

    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
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
    use super::*;

    #[test]
    fn test_max_profit_v1() {
        /*
            输入：[7,1,5,3,6,4]
            输出：5
            解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
                注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
        */
        assert_eq!(Solution::max_profit_v1(vec![7, 1, 5, 3, 6, 4]), 5);

        /*
            输入：prices = [7,6,4,3,1]
            输出：0
            解释：在这种情况下, 没有交易完成, 所以最大利润为 0。
        */
        assert_eq!(Solution::max_profit_v1(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_v2() {
        assert_eq!(Solution::max_profit_v2(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_v2(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_v3() {
        assert_eq!(Solution::max_profit_v3(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_v3(vec![7, 6, 4, 3, 1]), 0);
    }
}
