use super::Solution;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        #[inline]
        fn dfs(x: usize, g: &Vec<Vec<usize>>, vis: &mut Vec<bool>) -> i64 {
            vis[x] = true; // 避免重复访问同一个点
            let mut size = 1;
            for &y in &g[x] {
                if !vis[y] {
                    size += dfs(y, g, vis);
                }
            }
            size
        }

        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x); // 建图
        }

        let mut ans = 0;
        let mut total = 0;
        let mut vis = vec![false; n];

        for i in 0..n {
            // 未访问的点：说明找到了一个新的连通块
            if !vis[i] {
                let size = dfs(i, &g, &mut vis);
                ans += size * total;

                total += size;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 0);
        assert_eq!(
            Solution::count_pairs(
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
            ),
            14
        );
    }
}
