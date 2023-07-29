pub struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/single-number/
    ///
    /// 只出现一次的数字
    ///
    /// ```
    /// // 4 : 00000000 00000000 00000000 00000100 XOR
    /// // 1 : 00000000 00000000 00000000 00000001 XOR
    /// // 5 : 00000000 00000000 00000000 00000101 XOR
    /// // 2 : 00000000 00000000 00000000 00000010 XOR
    /// // 7 : 00000000 00000000 00000000 00000111 XOR
    /// // 1 : 00000000 00000000 00000000 00000001 XOR
    /// // 6 : 00000000 00000000 00000000 00000110 XOR
    /// // 2 : 00000000 00000000 00000000 00000010 XOR
    /// // 4 : 00000000 00000000 00000000 00000100 =
    /// ```
    ///
    /// fold() 方法是一个迭代器递归器，将一个初始值 0 和一个闭包函数作为参数传入，对迭代器中的每个元素进行聚合计算，最终返回一个最终结果值。
    /// acc 是累加器，初始值为 0。x 是迭代器当前元素的值。
    /// 函数中使用了按位异或（XOR）操作符 ^ 对累加器和当前元素进行异或运算，并将结果作为下一次迭代的累加器值传入。
    /// 最终，函数返回的是累加器的值，即只出现一次的数字。
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| {
            println!("acc: {:?}", acc);
            println!("x: {:?}", x);
            println!("acc ^ x: {:?}", acc ^ x);
            acc ^ x
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_single_number() {
        // assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        // assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
