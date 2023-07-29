#![allow(unused)]
struct Solution;

impl Solution {
    /// 4的幂
    /// 其中 0x2aaaaaaa 是二进制下 10101010101010101010101010101010
    /// 代码的实现相比于判断 3 的幂稍微复杂了一些，需要利用两个条件：
    /// n 是 2 的幂，即 n & (n - 1) == 0；
    /// n 的二进制表示中，1 出现在奇数位上，即 n & 0x2aaaaaaa == 0。
    /// 这里的第二个条件相当于利用了 4 的幂的二进制表示中 1 出现在奇数位上这个性质。
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & 0x2aaaaaaa == 0
    }
}
