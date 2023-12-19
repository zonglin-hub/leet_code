//! 括号生成

use super::Solution;

impl Solution {
    /// 下面让我详细解释一下 DFS 的思路。
    /// 题目要求生成有效的括号组合，那么我们从最简单的情况考虑，例如生成一对括号的有效组合，这个时候只有一种情况，就是 "()"。
    /// 如果我们继续往下加括号，假设加一对括号，那么新的有效组合就可以通过在原有的组合基础上加上一对括号。
    /// 例如一个有效组合是 "()"，那么在它的左右两侧分别加上一对括号，则新的两个有效组合分别为 "()()" 和 "(())"。
    /// 再继续往下加括号，可以发现，新的有效组合其实就可以通过在原有的组合基础上加上一对括号。
    /// 因此，这道题的解法就是 DFS，以当前字符串为基础继续添加括号并验证其合法性，直到添加完 n 对括号。
    /// 在 DFS 的过程中，我们需要记录剩余的左右括号数量。
    /// 为了保证每添加一对括号，产生的新组合都是唯一的，我们需要限制添加的顺序，即我们可以在剩余的左右括号数量中，只在剩余左括号数量大于剩余右括号数量时，添加右括号。
    pub fn generate_parenthesis_v1(n: i32) -> Vec<String> {
        #[inline]
        fn dfs(l: i32, r: i32, cur: &mut Vec<char>, res: &mut Vec<String>) {
            if l == 0 && r == 0 {
                res.push(cur.iter().collect())
            }

            if l > 0 {
                cur.push('(');
                dfs(l - 1, r, cur, res);
                cur.pop();
            }

            if r > l {
                cur.push(')');
                dfs(l, r - 1, cur, res);
                cur.pop();
            }
        }

        let mut res = vec![];
        dfs(n, n, &mut vec![], &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_generate_parenthesis_v1() {
        assert_eq!(Solution::generate_parenthesis_v1(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis_v1(0), vec![""]);
        assert_eq!(Solution::generate_parenthesis_v1(-1), Vec::<String>::new());
        assert_eq!(
            Solution::generate_parenthesis_v1(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
