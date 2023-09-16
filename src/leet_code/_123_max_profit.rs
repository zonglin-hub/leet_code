//! 买卖股票的最佳时机 III
//!

use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
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
