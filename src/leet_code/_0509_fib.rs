//! 斐波那契数

use super::Solution;

impl Solution {
    /// 这是一个计算斐波那契数列的函数。输入参数n为需要计算的数列项数，返回值为第n项的值。
    /// 该函数的实现方法是使用fold函数，将0到n-1的值进行迭代，每一步计算得到当前项的值，最终返回第n项的结果。
    /// 具体实现是通过元组t来存储前一项和当前项的值，将当前项的值与前一项的值相加得到新的当前项的值，并将新的当前项的值和前一项的值作为新的元组t返回。
    /// 最后通过.0来获取元组中的第一个值，即第n项的值。
    pub fn fib_v1(n: i32) -> i32 {
        (0..n).fold((0, 1), |t, _| (t.1, t.0 + t.1)).0
    }

    /// 这是一个计算斐波那契数列的函数。输入参数n为需要计算的数列项数，返回值为第n项的值。
    /// 该函数的实现方法是通过模式匹配判断n的值，当n为0或1时，直接返回n。当n大于1时，将n减2和减1分别作为参数递归调用函数本身，将两个递归调用的结果相加返回，即为第n项的值。
    /// 需要注意的是，该函数使用了Self::fib_v2的形式来调用本身，表示调用同一结构体中的另一个同名函数。
    pub fn fib_v2(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            _ => Self::fib_v2(n - 2) + Self::fib_v2(n - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_fib_v1() {
        assert_eq!(Solution::fib_v1(2), 1);
        assert_eq!(Solution::fib_v1(3), 2);
        assert_eq!(Solution::fib_v1(4), 3);
    }

    #[test]
    fn test_fib_v2() {
        assert_eq!(Solution::fib_v2(2), 1);
        assert_eq!(Solution::fib_v2(3), 2);
        assert_eq!(Solution::fib_v2(4), 3);
    }
}
