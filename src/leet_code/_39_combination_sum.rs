use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn recursion(nums: &Vec<i32>, tar: i32, pos: usize) -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            for i in pos..nums.len() {
                let cur = nums[i];

                // if cur > tar {
                //     continue;
                // } else if cur == tar {
                //     ans.push(vec![cur]);
                // } else {
                //     for mut v in recursion(nums, tar - cur, i) {
                //         v.push(cur);
                //         ans.push(v);
                //     }
                // }
                // 等同
                match cur.cmp(&tar) {
                    Ordering::Greater => continue,
                    Ordering::Equal => ans.push(vec![cur]),
                    Ordering::Less => {
                        for mut v in recursion(nums, tar - cur, i) {
                            v.push(cur);
                            ans.push(v);
                        }
                    }
                }
            }
            ans
        }

        recursion(&candidates, target, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![3, 2, 2], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]
        );
        // assert_eq!(
        //     Solution::combination_sum(vec![2], 1),
        //     vec![Vec::<i32>::new()]
        // );
    }
}
