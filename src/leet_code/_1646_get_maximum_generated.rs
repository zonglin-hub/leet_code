use super::Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut v = vec![0; n as usize + 1];
        v[1] = 1;
        for i in 2..v.len() {
            if i & 1 == 0 {
                v[i] = v[i / 2];
            } else {
                v[i] = v[i / 2] + v[i / 2 + 1];
            }
        }
        *v.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_get_maximum_generated() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(2), 1);
        assert_eq!(Solution::get_maximum_generated(3), 2);
        assert_eq!(Solution::get_maximum_generated(0), 0);
    }
}
