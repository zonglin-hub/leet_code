//! 只出现一次的数字。

use super::Solution;

impl Solution {
    /// 当两个二进制数进行按位异或（XOR）操作时，如果对应位的值相同，则结果为 0；如果对应位的值不同，则结果为 1。
    ///
    /// 4 : 00000000 00000000 00000000 00000100 XOR <br>
    /// 1 : 00000000 00000000 00000000 00000001 XOR <br>
    /// 5 : 00000000 00000000 00000000 00000101 XOR <br>
    /// 2 : 00000000 00000000 00000000 00000010 XOR <br>
    /// 7 : 00000000 00000000 00000000 00000111 XOR <br>
    /// 1 : 00000000 00000000 00000000 00000001 XOR <br>
    /// 6 : 00000000 00000000 00000000 00000110 XOR <br>
    /// 2 : 00000000 00000000 00000000 00000010 XOR <br>
    /// 4 : 00000000 00000000 00000000 00000100 =   <br>
    ///
    /// fold() 方法是一个迭代器递归器，将一个初始值 0 和一个闭包函数作为参数传入，对迭代器中的每个元素进行聚合计算，最终返回一个最终结果值。
    /// acc 是累加器，初始值为 0。x 是迭代器当前元素的值。
    /// 函数中使用了按位异或（XOR）操作符 ^ 对累加器和当前元素进行异或运算，并将结果作为下一次迭代的累加器值传入。
    /// 最终，函数返回的是累加器的值，即只出现一次的数字。
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| {
            // println!("acc: {:?}", acc);
            // println!("x: {:?}", x);
            // println!("acc ^ x: {:?}", acc ^ x);
            acc ^ x
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
        assert_eq!(Solution::single_number(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(Solution::single_number(vec![3, 3, 7, 7, 10, 11, 11]), 10);
    }
}
