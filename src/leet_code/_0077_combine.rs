use super::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(cur: i32, n: i32, k: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if tmp.len() + ((n - cur + 1) as usize) < k as usize {
                return;
            }

            if tmp.len() == k as usize {
                return ans.push(tmp.to_vec());
            }

            tmp.push(cur);

            dfs(cur + 1, n, k, tmp, ans);

            tmp.pop();

            dfs(cur + 1, n, k, tmp, ans);
        }

        let mut tmp = Vec::new();
        let mut ans = Vec::new();
        dfs(1, n, k, &mut tmp, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_combine() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
