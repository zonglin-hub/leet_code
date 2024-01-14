use super::Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut s = BTreeSet::new();
        for num in nums {
            s.insert(num);

            if s.len() > 3 {
                s.remove(s.clone().first().unwrap());
            }
        }
        if s.len() == 3 {
            *s.first().unwrap()
        } else {
            *s.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
