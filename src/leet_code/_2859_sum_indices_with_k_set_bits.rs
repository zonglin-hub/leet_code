use super::Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .fold(0, |acc, (i, &v)| acc + if i.count_ones() == k as u32 { v } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1), 13);
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2), 1);
    }
}
