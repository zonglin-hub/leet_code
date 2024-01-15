use super::Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        for i in 0..=nums.len() as i32 {
            let mut cnt = 0;
            for j in &nums {
                if *j >= i {
                    cnt += 1;
                }
            }
            if cnt == i {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_special_array() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
        assert_eq!(Solution::special_array(vec![3, 6, 7, 7, 0]), -1);
    }
}
