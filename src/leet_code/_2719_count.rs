use super::Solution;

impl Solution {
    #[allow(clippy::too_many_arguments)]
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        #[inline]
        fn dfs(
            n: usize,
            low: &[u8],
            high: &[u8],
            min_sum: i32,
            max_sum: i32,
            memo: &mut Vec<Vec<i32>>,
            i: usize,
            l_limit: bool,
            r_limit: bool,
            cur_sum: i32,
        ) -> i32 {
            if i == n {
                return if cur_sum >= min_sum && cur_sum <= max_sum { 1 } else { 0 };
            }
            if !l_limit && !r_limit {
                let mem = memo[i][cur_sum as usize];
                if mem >= 0 {
                    return mem;
                }
            }
            let lo = if l_limit { low[i] } else { b'0' };
            let hi = if r_limit { high[i] } else { b'9' };
            let mut res = 0;
            for d in lo..hi + 1 {
                if cur_sum + (d - b'0') as i32 <= max_sum {
                    res += dfs(
                        n,
                        low,
                        high,
                        min_sum,
                        max_sum,
                        memo,
                        i + 1,
                        l_limit && d == lo,
                        r_limit && d == hi,
                        cur_sum + (d - b'0') as i32,
                    );
                    res %= 1e9 as i32 + 7;
                }
            }
            if !l_limit && !r_limit {
                memo[i][cur_sum as usize] = res;
            }
            res
        }
        let n = num2.len();
        let num1 = format!("{:0>width$}", num1, width = n);
        dfs(
            n,
            num1.as_bytes(),
            num2.as_bytes(),
            min_sum,
            max_sum,
            &mut vec![vec![-1; max_sum as usize + 1]; n],
            0,
            true,
            true,
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_color_the_array() {
        assert_eq!(Solution::count("1".to_owned(), "12".to_owned(), 1, 8), 11);
        assert_eq!(Solution::count("1".to_owned(), "5".to_owned(), 1, 5), 5);
    }
}
