use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::k_sum(4, nums, target)
    }

    pub fn k_sum(k: i32, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < k as usize {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut nums = nums;
        nums.sort();

        if k == 2 {
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left < right {
                match (nums[left] + nums[right]).cmp(&target) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        results.push(vec![nums[left], nums[right]]);
                        left += 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                }
            }
        } else {
            for i in 0..nums.len() - k as usize + 1 {
                if nums[i] * k > target {
                    break;
                }

                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                let sub_results = Self::k_sum(k - 1, nums[i + 1..].to_vec(), target - nums[i]);

                for mut r in sub_results {
                    r.push(nums[i]);
                    results.push(r);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![1, 2, -1, -2], vec![0, 2, 0, -2], vec![0, 1, 0, -1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
    }
}
