use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        recursion(&candidates, target, 0)
    }
}

pub fn recursion(nums: &Vec<i32>, tar: i32, pos: usize) -> Vec<Vec<i32>> {
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
