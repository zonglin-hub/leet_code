use super::Solution;

impl Solution {
    pub fn maximum_sum_of_heights_2865(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut prefix = vec![0; n];
        let mut stack = vec![];
        for i in 0..n {
            while !stack.is_empty() && max_heights[i] < max_heights[stack[stack.len() - 1]] {
                stack.pop();
            }
            if stack.is_empty() {
                prefix[i] = (i + 1) as i64 * max_heights[i] as i64;
            } else {
                prefix[i] = prefix[stack[stack.len() - 1]]
                    + (i - stack[stack.len() - 1]) as i64 * max_heights[i] as i64;
            }
            stack.push(i);
        }

        let mut res = 0;
        let mut suffix = vec![0; n];
        let mut stack = vec![];
        for i in (0..n).rev() {
            while !stack.is_empty() && max_heights[i] < max_heights[stack[stack.len() - 1]] {
                stack.pop();
            }
            if stack.is_empty() {
                suffix[i] = (n - i) as i64 * max_heights[i] as i64;
            } else {
                suffix[i] = suffix[stack[stack.len() - 1]]
                    + (stack[stack.len() - 1] - i) as i64 * max_heights[i] as i64;
            }
            stack.push(i);
            res = res.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::maximum_sum_of_heights_2865(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights_2865(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights_2865(vec![3, 2, 5, 5, 2, 3]), 18);
    }
}
