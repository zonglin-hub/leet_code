//! 括号的分数

pub struct Solution;

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
                let last = score.pop().unwrap();
                // 这是一个复合赋值语句，把 *score.last_mut().unwrap() 的值加上 if last == 0 { 1 } else { last << 1 } 的值，再赋回 *score.last_mut().unwrap()。
                *score.last_mut().unwrap() += if last == 0 { 1 } else { last << 1 };
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

    #[test]
    fn test_bit_arithmetic() {
        // 这是一个位运算符号 << 的示例，表示将一个二进制数向左移动指定的位数，类似于十进制数的乘以2的n次方。例如：
        // 0 左移 1 位，结果为 0，因为二进制中的 0 向左移动 1 位仍然是 0。
        // 1 左移 1 位，结果为 2，因为二进制中的 1 向左移动 1 位变成了 10，而 10 表示的十进制数就是 2。
        // 2 左移 1 位，结果为 4，因为二进制中的 10 向左移动 1 位变成了 100，而 100 表示的十进制数就是 4。
        // 在这个算法中，如果遇到嵌套的括号，会将内层括号的分数向左移动一位（乘以2），然后加到外层括号的分数上。因此，last << 1 的操作就是将 last 值乘以2，相当于内层括号分数乘以2加到外层括号分数上去。
        assert_eq!(0 << 1, 0);
        assert_eq!(1 << 1, 2);
        assert_eq!(2 << 1, 4);

        // 这是一个位运算符号 >> 的示例，表示将一个二进制数向右移动指定的位数，类似于十进制数的除以2的n次方。例如：
        // 0 右移 1 位，结果为 0，因为二进制中的 0 向右移动 1 位仍然是 0。
        // 1 右移 1 位，结果为 0，因为二进制中的 1 向右移动 1 位变成了 0，而 0 表示的十进制数就是 0。
        // 2 右移 1 位，结果为 1，因为二进制中的 10 向右移动 1 位变成了 1，而 1 表示的十进制数就是 1。
        // 3 右移 1 位，结果为 1，因为二进制中的 11 向右移动 1 位变成了 1，而 1 表示的十进制数就是 1。
        // 4 右移 1 位，结果为 2，因为二进制中的 100 向右移动 1 位变成了 10，而 10 表示的十进制数就是 2。
        // 在这个算法中，如果遇到嵌套的括号，会将外层括号的分数向右移动一位（除以2），然后减去内层括号的分数，因此 last >> 1 的操作就是将 last 值除以2，相当于外层括号分数
        assert_eq!(0 >> 1, 0);
        assert_eq!(1 >> 1, 0);
        assert_eq!(2 >> 1, 1);
        assert_eq!(3 >> 1, 1);
        assert_eq!(4 >> 1, 2);
    }
}
