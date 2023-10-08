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
        let mut carr = Vec::with_capacity(100);
        let mut ans = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                carr.push(i as i32);
            } else {
                carr.pop();
                if carr.is_empty() {
                    carr.push(i as i32);
                // 否则，获取Vec中最后一个元素的索引
                } else {
                    let carr = carr.last().expect("");
                    // 计算当前有效括号长度
                    ans = ans.max(i as i32 - carr);
                }
            }
        }
        ans
    }
}
