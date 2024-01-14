//! 最长有效括号

use super::Solution;

impl Solution {
    /// 计算给定字符串s中有效括号对的最大长度。
    ///
    /// 它使用两个栈，carr和ans来跟踪开括号和闭括号的索引。
    /// 函数遍历输入字符串中的每个字符，对于每个字符，它检查它是否是一个开括号，然后将其索引推入carr栈。
    /// 如果它是一个闭括号，它从carr栈中弹出顶部元素，并更新ans变量为当前有效括号对长度与当前索引和最后一个开括号索引之差的最大值。
    /// 这样，我们就可以跟踪并找到最长的有效括号对子串。最后，函数返回最长的有效括号对长度。
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut carr = vec![];
        let mut ans = 0;

        carr.push(-1);

        for (i, c) in s.chars().enumerate() {
            let i = i as i32;

            if c == '(' {
                carr.push(i);
            } else {
                carr.pop();

                if carr.is_empty() {
                    carr.push(i);
                } else {
                    let carr = carr.last().unwrap();
                    ans = ans.max(i - carr);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
    }
}
