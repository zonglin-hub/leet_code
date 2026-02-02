use crate::leet_code::Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();

        let mut ans = 0;
        for j in 0..m {
            for i in 1..n {
                if strs[i].as_bytes()[j] < strs[i - 1].as_bytes()[j] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
        assert_eq!(Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]), 0);
        assert_eq!(
            Solution::min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
    }
}
