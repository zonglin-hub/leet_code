//! 买卖股票的最佳时机。

use super::Solution;

impl Solution {
    pub fn max_profit_121_v1(prices: Vec<i32>) -> i32 {
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

impl Solution {
    pub fn max_profit_122(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|x| x[1] - x[0])
            .filter(|&x| x > 0)
            .sum()
    }
}

impl Solution {
    pub fn max_profit_123(prices: Vec<i32>) -> i32 {
        let (mut buy0, mut sell0) = (-prices[0], 0);
        let (mut buy1, mut sell1) = (-prices[0], 0);
        prices.iter().skip(1).for_each(|&v| {
            buy0 = buy0.max(-v);
            sell0 = sell0.max(v + buy0);
            buy1 = buy1.max(sell0 - v);
            sell1 = sell1.max(v + buy1);
        });
        sell1
    }
}
