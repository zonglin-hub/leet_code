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
