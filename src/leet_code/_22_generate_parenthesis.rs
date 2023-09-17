//! 括号生成

use super::Solution;

impl Solution {
    #[doc = include_str!("../doc/leet_code/_22_generate_parenthesis.md")]
    pub fn generate_parenthesis_v1(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut cur = Vec::new();
        Self::dfs(n, n, &mut cur, &mut res);
        res
    }

    fn dfs(l: i32, r: i32, cur: &mut Vec<char>, res: &mut Vec<String>) {
        if l == 0 && r == 0 {
            res.push(cur.iter().collect())
        } else {
            if l > 0 {
                cur.push('(');
                Self::dfs(l - 1, r, cur, res);
                cur.pop();
            }
            if r > l {
                cur.push(')');
                Self::dfs(l, r - 1, cur, res);
                cur.pop();
            }
        }
    }
}
