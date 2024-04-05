use super::Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let origin_len = nums.len();
        let mut target = vec![0; origin_len];

        index.into_iter().zip(nums).for_each(|(index, element)| {
            if target[index as usize] == -1 {
                target[index as usize] = element;
            } else {
                target.insert(index as usize, element);
            }
        });

        target.truncate(origin_len);
        target
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    }
}
