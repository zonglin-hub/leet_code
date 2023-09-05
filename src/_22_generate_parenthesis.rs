pub struct Solution;

impl Solution {
    /// 括号生成
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"])
    }
}
