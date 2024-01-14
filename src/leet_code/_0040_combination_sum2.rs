use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                        for mut af in sol(&candidates[i + 1..], target - *candidate) {
                            let mut tmp = sub.clone();
                            tmp.append(&mut af);
                            ans.push(tmp);
                        }
                    }
                }
            }
            ans
        }

        candidates.sort();
        sol(&candidates[..], target)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_combination_sum2() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
