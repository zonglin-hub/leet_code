//! 2 的幂
//! 相似题型：231 | 326 | 342

use super::Solution;

impl Solution {
    pub fn is_power_of_two_231(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }

    pub fn is_power_of_two_v1(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n == 0 || (n & 1) != 0 {
            return false;
        }
        Self::is_power_of_two_231(n / 2)
    }
}

impl Solution {
    /// 3 的幂
    /// 其中 1162261467 是 int 类型范围内最大的 3 的幂，是 3^19
    /// 先判断 n 是否大于 0，然后判断 1162261467 是否能被 n 整除即可
    pub fn is_power_of_three_326(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
}

impl Solution {
    /// 其中 0x2aaaaaaa 是二进制下 10101010101010101010101010101010
    /// 代码的实现相比于判断 3 的幂稍微复杂了一些，需要利用两个条件：
    /// n 是 2 的幂，即 n & (n - 1) == 0；
    /// n 的二进制表示中，1 出现在奇数位上，即 n & 0x2aaaaaaa == 0。
    /// 这里的第二个条件相当于利用了 4 的幂的二进制表示中 1 出现在奇数位上这个性质。
    pub fn is_power_of_four_342(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & 0x2aaaaaaa == 0
    }
}
