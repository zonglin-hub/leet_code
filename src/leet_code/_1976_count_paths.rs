use std::{cmp::Ordering, collections::BinaryHeap};

use super::Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct Pair {
            cost: i64,
            node: usize,
        }
        const MOD: i64 = 1_000_000_007;

        let mut e: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n as usize];
        for road in roads {
            let x = road[0] as usize;
            let y = road[1] as usize;
            let t = road[2];
            e[x].push((y, t));
            e[y].push((x, t));
        }
        let mut dis: Vec<i64> = vec![i64::MAX; n as usize];
        let mut ways: Vec<i64> = vec![0; n as usize];

        let mut q = BinaryHeap::new();
        q.push(Pair { cost: 0, node: 0 });
        dis[0] = 0;
        ways[0] = 1;

        while let Some(Pair { cost: t, node: u }) = q.pop() {
            if t > dis[u] {
                continue;
            }

            for &(v, w) in &e[u] {
                match (t + (w as i64)).cmp(&dis[v]) {
                    Ordering::Equal => ways[v] = (ways[v] + ways[u]) % MOD,
                    Ordering::Less => {
                        dis[v] = t + (w as i64);
                        ways[v] = ways[u];
                        q.push(Pair { cost: t + (w as i64), node: v });
                    }
                    Ordering::Greater => (),
                }

                // if t + (w as i64) < dis[v] {
                // } else if t + (w as i64) == dis[v] {
                // }
            }
        }
        ways[(n - 1) as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::count_paths(
                7,
                vec![
                    vec![0, 6, 7],
                    vec![0, 1, 2],
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![6, 3, 3],
                    vec![3, 5, 1],
                    vec![6, 5, 1],
                    vec![2, 5, 1],
                    vec![0, 4, 5],
                    vec![4, 6, 2]
                ]
            ),
            3
        );
        assert_eq!(Solution::count_paths(2, vec![vec![1, 0, 10]]), 1);
    }
}
