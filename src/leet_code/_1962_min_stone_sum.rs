use std::collections::BinaryHeap;

use super::Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::from(piles);
        for _i in 0..k {
            let top = h.pop().unwrap();
            h.push((top + 1) / 2);

            if h.peek() == Some(&1) {
                break;
            }
        }

        h.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_stone_sum() {
        assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
        assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
    }
}
