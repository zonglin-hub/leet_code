//! 括号的分数

use crate::Solution;

impl Solution {
    // 栈
    pub fn score_of_parentheses_v1(s: String) -> i32 {
        // 创建一个用于存储括号的Vec
        let mut score = Vec::with_capacity((s.len() >> 1) + 1);
        // 将初始分数设置为0
        score.push(0);

        // 遍历字符串，如果遇到左括号，则将分数加1，否则将分数减去上一个分数的值，并将分数加到最后一个分数中
        for c in s.bytes() {
            if c == b'(' {
                score.push(0);
            } else {
                let last = score.pop().expect("");
                // 这是一个复合赋值语句，把 *score.last_mut().expect("") 的值加上 if last == 0 { 1 } else { last << 1 } 的值，再赋回 *score.last_mut().expect("")。
                *score.last_mut().expect("") += if last == 0 { 1 } else { last << 1 };
            }
        }

        // 返回最后一个分数的值
        score[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_parentheses_v1() {
        /*
            输入： "()"
            输出： 1
        */
        assert_eq!(Solution::score_of_parentheses_v1("()".to_string()), 1);

        /*
            输入： "(())"
            输出： 2
        */
        assert_eq!(Solution::score_of_parentheses_v1("(())".to_string()), 2);

        /*
            输入： "()()"
            输出： 2
        */
        assert_eq!(Solution::score_of_parentheses_v1("()()".to_string()), 2);

        /*
            输入： "(()(()))"
            输出： 6
        */
        assert_eq!(Solution::score_of_parentheses_v1("(()(()))".to_string()), 6);
    }
}
