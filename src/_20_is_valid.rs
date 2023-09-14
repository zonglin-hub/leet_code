//! 有效的括号

use crate::types::base_type::Solution;

impl Solution {
    /// 这是一道典型的栈的问题。
    ///
    /// 题目要求判断字符串中的括号是否匹配，如果匹配返回true，否则返回false。
    /// 可以使用栈来解决这个问题，遍历字符串中的每个字符，遇到左括号则将其推入栈中，遇到右括号则将栈顶元素出栈与其匹配，如果不匹配则返回false，如果匹配则继续遍历。
    /// 如果遍历完成后栈为空，则说明所有括号都匹配，返回true，否则返回false。
    /// 对于这里的代码，采用了函数式编程的思路，使用fold来将字符逐个处理，如果当前字符是左括号，则将其压入栈中；
    /// 如果当前字符是右括号，则将栈顶元素出栈与其匹配，如果不匹配就将之前出栈的元素再次压入栈中。
    /// 当遍历完成后，如果栈为空，则表明括号匹配成功，否则匹配失败。
    /// 这样，时间复杂度为O(n)，空间复杂度为O(n)，其中n为字符串的长度。
    pub fn is_valid_v1(s: String) -> bool {
        s.into_bytes() // 将字符串s转换为字节数组。
            .drain(..) // 迭代字节数组中的每个元素，返回一个迭代器并清空原始的字节数组。
            .fold(Vec::with_capacity(25), |mut s, x| {
                // 将迭代器中的每个元素 x 与一个初始值 Vec::with_capacity(25)（即容量为 25 的初始空 Vec）进行累加操作，通过匹配栈中的上一个元素来判断括号序列是否有效。
                // println!("s: {:?}, x: {}\n", s, x);
                // s: [], x: 40

                // s: [40], x: 41

                // s: [], x: 91

                // s: [91], x: 93

                // s: [], x: 123

                // s: [123], x: 125

                match (s.pop(), x) {
                    (Some(b'['), b']') | (Some(b'('), b')') | (Some(b'{'), b'}') => (), // 如果匹配成功，则不做任何处理；
                    (Some(a), b) => {
                        // 如果匹配失败，则先将上一个元素弹出，再将当前元素和上一个元素都压回栈中；
                        s.push(a);
                        s.push(b)
                    }
                    (None, b) => {
                        // 如果当前栈为空，则直接将当前元素压入栈中。
                        s.push(b);
                    }
                };
                s
            })
            .is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_v1() {
        /*
            输入：s = "()"
            输出：true
        */
        assert!(Solution::is_valid_v1(String::from("()")));

        /*
            输入：s = "()[]{}"
            输出：true
        */
        assert!(Solution::is_valid_v1(String::from("()[]{}")));

        /*
            输入：s = "(]"
            输出：false
        */
        assert_eq!(Solution::is_valid_v1(String::from("(]")), false);
    }
}
