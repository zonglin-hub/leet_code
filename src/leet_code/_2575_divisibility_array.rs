use super::Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut cur = 0;
        let m = m as i64;
        let mut res = vec![];

        for c in word.chars() {
            cur = (cur * 10 + (c.to_digit(10).unwrap() as i64)) % m;
            res.push(if cur == 0 { 1 } else { 0 });
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::divisibility_array("998244353".to_owned(), 3),
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
        assert_eq!(Solution::divisibility_array("1010".to_owned(), 10), vec![0, 1, 0, 1]);
    }
}
