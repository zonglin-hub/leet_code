use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        Self::sol(&candidates[..], target)
    }
    fn sol(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut unique = i32::MAX;
        for (i, candidate) in candidates.iter().enumerate() {
            if *candidate == unique {
                continue;
            } else {
                unique = *candidate;
            }

            let sub = vec![*candidate];

            match candidate.cmp(&target) {
                Ordering::Greater => break,
                Ordering::Equal => ans.push(sub),
                Ordering::Less => {
                    for mut af in Self::sol(&candidates[i + 1..], target - *candidate) {
                        let mut tmp = sub.clone();
                        tmp.append(&mut af);
                        ans.push(tmp);
                    }
                }
            }
        }
        ans
    }
}
