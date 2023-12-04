//! 收集树中金币

use std::collections::{HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();

        let mut d = edges
            .iter()
            .fold(vec![HashSet::<i32>::new(); n], |mut acc, v| {
                acc[v[0] as usize].insert(v[1]);
                acc[v[1] as usize].insert(v[0]);
                acc
            });

        let mut q = (0..n)
            .filter(|i| d[*i].len() == 1 && coins[*i] == 0)
            .collect::<VecDeque<_>>();

        while let Some(x) = q.pop_front() {
            d[x].clone().into_iter().for_each(|y| {
                d[y as usize].remove(&(x as i32));

                if d[y as usize].len() == 1 && coins[y as usize] == 0 {
                    q.push_back(y as usize);
                }
            })
        }

        ((0..2)
            .fold(d, |mut acc, _| {
                acc.iter()
                    .enumerate()
                    .filter(|(_, d)| d.len() == 1)
                    .map(|(i, _)| i)
                    .collect::<VecDeque<_>>()
                    .into_iter()
                    .for_each(|x| {
                        acc[x].clone().iter().for_each(|y| {
                            acc[*y as usize].remove(&(x as i32));
                        });
                        acc[x].clear();
                    });

                acc
            })
            .iter()
            .filter(|d| !d.is_empty())
            .count()
            .max(1)
            .saturating_sub(1)
            * 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_collect_the_coins() {
        assert_eq!(
            Solution::collect_the_coins(
                vec![1, 0, 0, 0, 0, 1],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
            ),
            2
        );
        assert_eq!(
            Solution::collect_the_coins(
                vec![0, 0, 0, 1, 1, 0, 0, 1],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![5, 6],
                    vec![5, 7]
                ]
            ),
            2
        );
    }
}
