use super::Solution;

impl Solution {
    pub fn plus_one(mut res: Vec<i32>) -> Vec<i32> {
        for i in (0..res.len()).rev() {
            if res[i] != 9 {
                res[i] += 1;
                return res;
            }
            res[i] = 0;
        }
        res.insert(0, 1);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 1, 2, 3]), vec![4, 1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
