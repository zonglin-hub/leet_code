#![allow(unused)]
struct Solution;

impl Solution {
    /// 3 的幂
    /// 其中 1162261467 是 int 类型范围内最大的 3 的幂，是 3^19
    /// 先判断 n 是否大于 0，然后判断 1162261467 是否能被 n 整除即可
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
}
