pub struct Solution;

impl Solution {
    /// 不重叠回文子字符串的最大数目
    ///
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_palindromes() {
        assert_eq!(Solution::max_palindromes(String::from("abaccdbbd"), 3), 2);
        assert_eq!(Solution::max_palindromes(String::from("adbcda"), 2), 0);
        assert_eq!(
            Solution::max_palindromes(String::from("fttfjofpnpfydwdwdnns"), 2),
            4
        );
    }

    #[test]
    fn test() {
        let n = 5;
        // 这里的 0..2 * n - 1 是一个 range，表示一个从0到2*n-2的半开区间。具体来说，如果 n=5，那么这个 range 会包含如下数字序列：0, 1, 2, 3, 4, 5, 6, 7，而不包含8。
        // 由于本算法中需要遍历所有的子串，因此遍历到的子串的数目是 2n-1 个。其中，前 n 个是以每个字符为中心的单字符子串，后 n-1 个是以每个字符间隙为中心的双字符子串。例如，对于字符串 "abc"，子串遍历顺序为：a, b, c, ab, bc。
        // 因此，这里的 0..2 * n - 1 范围对应的是遍历所有子串的范围。
        for i in 0..2 * n - 1 {
            println!("{:?}", i)
        }
    }
}
