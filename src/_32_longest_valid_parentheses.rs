use crate::types::base_type::Solution;

impl Solution {
    /// 计算给定字符串s中有效括号对的最大长度。
    ///
    /// 它使用两个栈，carr和ans来跟踪开括号和闭括号的索引。
    /// 函数遍历输入字符串中的每个字符，对于每个字符，它检查它是否是一个开括号，然后将其索引推入carr栈。
    /// 如果它是一个闭括号，它从carr栈中弹出顶部元素，并更新ans变量为当前有效括号对长度与当前索引和最后一个开括号索引之差的最大值。
    /// 这样，我们就可以跟踪并找到最长的有效括号对子串。最后，函数返回最长的有效括号对长度。
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // 创建一个Vec容量为100的空Vec
        let mut carr = Vec::with_capacity(100);
        // 初始化最长有效括号的长度为0
        let mut ans = 0;
        // 遍历字符串中的每一个字符
        for (i, c) in s.chars().enumerate() {
            // 如果当前字符为左括号，则将其索引转换为整数
            if c == '(' {
                carr.push(i as i32);
            // 否则，将其索引转换为整数，并删除Vec中的最后一个元素
            } else {
                carr.pop();
                // 如果Vec为空，则将其索引转换为整数，并将其索引转换为整数加上当前有效括号的长度
                if carr.is_empty() {
                    carr.push(i as i32);
                // 否则，获取Vec中最后一个元素的索引
                } else {
                    let carr = carr.last().unwrap();
                    // 计算当前有效括号长度
                    ans = ans.max(i as i32 - carr);
                }
            }
        }
        // 返回最长有效括号的长度
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
