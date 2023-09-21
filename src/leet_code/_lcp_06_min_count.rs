use super::Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in coins {
            sum += (i + 1) / 2
        }
        sum
    }
}
