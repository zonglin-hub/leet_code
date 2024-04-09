use super::Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let n = nums.len();

        for (i, _) in nums.iter().enumerate().take(n) {
            for (j, val) in nums.iter().enumerate().take(n) {
                if *val == key && i32::abs(i as i32 - j as i32) <= k {
                    res.push(i as i32);
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
    }
}
