use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        fn dfs(x: usize, f: usize, g: &Vec<Vec<usize>>) -> i32 {
            let mut cnt = 1;
            for &y in &g[x] {
                if y != f {
                    cnt += dfs(y, x, g);
                }
            }
            cnt
        }

        let restricted = restricted.into_iter().collect::<HashSet<i32>>();
        let mut dp = vec![vec![]; n as usize];

        for i in edges {
            let x = i[0];
            let y = i[1];

            if !restricted.contains(&x) && !restricted.contains(&y) {
                dp[x as usize].push(y as usize);
                dp[y as usize].push(x as usize);
            }
        }

        dfs(0, 0, &dp)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![vec![0, 1], vec![1, 2], vec![3, 1], vec![4, 0], vec![0, 5], vec![5, 6]],
                vec![4, 5]
            ),
            4
        );
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![vec![0, 1], vec![0, 2], vec![0, 5], vec![0, 4], vec![3, 2], vec![6, 5]],
                vec![4, 2, 1]
            ),
            3
        );
    }
}
