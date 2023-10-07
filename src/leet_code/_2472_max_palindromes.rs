//! 不重叠回文子字符串的最大数目

use super::Solution;

impl Solution {
    /// 将字符串转换为字节数组，记录字符串长度n。
    /// 定义一个长度为n+1的数组v，`v[i]` 表示前缀字符串s[0..i]中可以构成的最大回文串数。
    /// 遍历所有的子串，对于每个子串s[l..r]：-> 如果子串长度小于k，不做处理。 -> 如果子串可以构成回文串，更新 `v[r+1]` 为 `v[l]+1`。 -> 否则，左右扩展子串，继续判断是否可以构成回文串。
    /// 最终返回最大回文串数。
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut v = vec![0; n + 1];

        for i in 0..2 * n - 1 {
            let mut l = i / 2;
            let mut r = l + i % 2;

            v[l + 1] = std::cmp::max(v[l + 1], v[l]);

            while r < n && s[l] == s[r] {
                if r - l + 1 >= k as usize {
                    v[r + 1] = std::cmp::max(v[r + 1], v[l] + 1);
                    break;
                }

                if l == 0 {
                    break;
                }

                l -= 1;
                r += 1;
            }
        }

        v[n]
    }
}
