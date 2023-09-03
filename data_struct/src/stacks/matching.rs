//! 栈 实现 括号匹配 () [] {}
//!

#![allow(unused)]

use super::stack::Stack;

pub struct Solution;

impl Solution {
    /// 括号匹配
    pub fn is_matched(s: &str) -> bool {
        let mut char_list = Vec::new();
        for c in s.chars() {
            char_list.push(c);
        }

        let mut index = 0;

        // 括 号 是 否 匹 配 (平衡)标示
        let mut balance = true;

        // 使 用 前 面 实 现 的 栈
        let mut stack = Stack::new();
        while index < char_list.len() && balance {
            let c = char_list[index];

            if c == '(' {
                // 如 果 为 开 符 号 ， 入栈
                stack.push(c);
            } else {
                // 如 果 为 闭 符 号 ， 判 断 栈 是 否 为 空
                if stack.is_empty() {
                    // 为 空 则 不 平 衡
                    balance = false;
                } else {
                    let _r = stack.pop();
                }
            }
            index += 1;
        }
        // 平 衡 且 栈 为 空 ， 括 号 表 达 式 才 是 匹 配 的
        balance && stack.is_empty()
    }

    /// 同时检测多种开闭符号是否匹配
    fn par_match(open: char, close: char) -> bool {
        let opens = "([{";
        let closers = ")]}";
        opens.find(open) == closers.find(close)
    }

    fn is_matched_v2(s: &str) -> bool {
        let mut char_list = Vec::new();
        // 遍 历 字 符 串 字 符 ， 压 入 字 符 串 中
        for c in s.chars() {
            char_list.push(c);
        }

        let mut index = 0;
        let mut balance = true;
        let mut stack = Stack::new();

        while index < char_list.len() && balance {
            let c = char_list[index];
            // 同 时 判 断 三 种 开 符
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    balance = false;
                } else {
                    // 比 较 当 前 括 号 和 栈 顶 括 号 是 否 匹
                    let top = stack.pop().unwrap();
                    if !Self::par_match(top, c) {
                        balance = false;
                    }
                }
            }
            index += 1;
        }

        balance && stack.is_empty()
    }

    fn is_matched_v3(s: &str) -> bool {
        let mut char_list = Vec::new();
        for c in s.chars() {
            char_list.push(c);
        }

        let mut index = 0;
        let mut balance = true;
        let mut stack = Stack::new();

        while index < char_list.len() && balance {
            let c = char_list[index];

            // 开符号入栈
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            }
            // 闭 符 号 则 判 断 是 否 平 衡
            if ')' == c || ']' == c || '}' == c {
                if stack.is_empty() {
                    balance = false;
                } else {
                    let top = stack.pop().unwrap();

                    if !Self::par_match(top, c) {
                        balance = false;
                    }
                }
            }
            // 非 括 号 直 接 跳 过
            index += 1;
        }
        balance && stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matched() {
        assert_eq!(Solution::is_matched("()(())"), true);
        assert_eq!(Solution::is_matched(")("), false);
    }

    #[test]
    fn test_is_matched_v2() {
        assert_eq!(Solution::is_matched_v2("(){}[]"), true);
        assert_eq!(Solution::is_matched_v2("(){)[}"), false);
    }

    #[test]
    fn test_is_matched_v3() {
        assert_eq!(Solution::is_matched_v3("(2+3){func}[abc]"), true);
        assert_eq!(Solution::is_matched_v3("(2+3)*(3-1"), false);
    }
}
