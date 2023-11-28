//! LCP 50. 宝石补给

use super::Solution;

impl Solution {
    pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        let mut gem = gem;
        for op in operations {
            let num = gem[op[0] as usize] / 2;
            gem[op[0] as usize] -= num;
            gem[op[1] as usize] += num;
        }
        *gem.iter().max().unwrap() - *gem.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{expected_sort_vec, Solution};

    #[test]
    fn test_give_gem() {
        assert_eq!(
            Solution::give_gem(
                vec![3, 1, 2],
                expected_sort_vec(vec![[0, 2], [2, 1], [2, 0]])
            ),
            2
        );
        assert_eq!(
            Solution::give_gem(
                vec![100, 0, 50, 100],
                expected_sort_vec(vec![[0, 2], [0, 1], [3, 0], [3, 0]])
            ),
            75
        );
        assert_eq!(
            Solution::give_gem(
                vec![0, 0, 0, 0],
                expected_sort_vec(vec![[1, 2], [3, 1], [1, 2]])
            ),
            0
        );
    }
}
