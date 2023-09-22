use super::Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.iter().fold(0, |sum, &x| (x + 1) / 2 + sum)
    }
}
