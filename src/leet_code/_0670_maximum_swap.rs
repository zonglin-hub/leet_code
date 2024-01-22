use super::Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s = num.to_string().chars().collect::<Vec<char>>();
        let n = s.len();
        let mut max_idx = n - 1;
        let (mut p, mut q) = (n, 0);

        for i in (0..n - 1).rev() {
            if s[i] > s[max_idx] {
                max_idx = i;
            } else if s[i] < s[max_idx] {
                p = i;
                q = max_idx;
            }
        }

        if p == n {
            return num;
        }

        s.swap(p, q);
        s.iter().collect::<String>().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_generate_trees() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
