use super::Solution;

impl Solution {
    /// 这个函数的作用是判断两个字符串是否互为字谜。
    /// 它使用了动态规划的思想，通过创建一个二维数组`f`来记录当前位置的匹配情况，然后依次计算每个位置的匹配情况。
    /// 如果在某个位置上匹配成功，那么就可以直接返回`true`。如果在某个位置上匹配失败，那么就可以直接返回`false`。
    /// 最终，函数返回`f[len][0][0]`，表示整个字符串是否互为字谜。
    ///
    /// 下面是函数的具体实现：
    ///
    /// 首先，函数判断两个字符串的长度是否相等，如果不相等，直接返回`false`。
    /// 然后，函数创建一个三维数组`f`，用于记录每个位置的匹配情况。其中，第一个维度表示字符串的长度，第二个维度表示当前位置的索引，第三个维度表示当前位置的子字符串长度。
    /// 接着，函数将两个字符串转换为字符向量。接下来，函数遍历两个字符串，计算每个位置的匹配情况，并将结果存储在`f[1]`中。
    /// 最后，函数使用动态规划的思想，依次计算每个位置的匹配情况。
    /// 对于每个位置，函数首先判断当前位置的子字符串是否与前一个位置的子字符串匹配，如果匹配，则将结果存储在`f[n]`中。
    /// 如果当前位置的子字符串与前一个位置的子字符串不匹配，则继续判断当前位置的子字符串是否与后一个位置的子字符串匹配，如果匹配，则将结果存储在`f[n]`中。
    /// 如果在某个位置上匹配成功，那么就可以直接返回`true`。如果在某个位置上匹配失败，那么就可以直接返回`false`。最终，函数返回`f[len][0][0]`，表示整个字符串是否互为字谜。
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let len = if s1.len() == s2.len() {
            s1.len()
        } else {
            return false;
        };

        let mut f = vec![vec![vec![false; len]; len]; len + 1];
        let (s1, s2) = (s1.chars().collect::<Vec<char>>(), s2.chars().collect::<Vec<char>>());

        s1.iter().enumerate().for_each(|(i, &x)| {
            s2.iter().enumerate().for_each(|(j, &y)| {
                f[1][i][j] = x == y;
            });
        });

        (1..=len).for_each(|n| {
            (0..=(len - n)).for_each(|i| {
                (0..=(len - n)).for_each(|j| {
                    for k in 1..n {
                        if (f[k][i][j] && f[n - k][i + k][j + k])
                            || (f[k][i][j + n - k] && f[n - k][i + k][j])
                        {
                            f[n][i][j] = true;
                            break;
                        }
                    }
                });
            });
        });

        f[len][0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_scramble() {
        assert!(Solution::is_scramble("a".to_string(), "a".to_string()));
        assert!(Solution::is_scramble("great".to_string(), "rgeat".to_string()));
        assert!(!Solution::is_scramble("abcde".to_string(), "caebd".to_string()));
    }
}
