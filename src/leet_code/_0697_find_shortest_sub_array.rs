use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt_left_len = HashMap::new();
        let mut degree = 0;

        for (i, val) in nums.iter().enumerate() {
            let cll = cnt_left_len.entry(val).or_insert([0, i, 1]);
            cll[0] += 1;
            cll[2] = i - cll[1] + 1;
            degree = degree.max(cll[0]);
        }
        cnt_left_len.values().filter(|arr| arr[0] == degree).map(|arr| arr[2]).min().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
    }
}
