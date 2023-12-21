use std::collections::VecDeque;

impl super::Solution {
    /// 使用 `VecDeque` 优化
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut prefix = vec![0; n];
        let mut stack = VecDeque::new();
        for i in 0..n {
            while !stack.is_empty() && max_heights[i] < max_heights[*stack.back().unwrap()] {
                stack.pop_back();
            }
            if stack.is_empty() {
                prefix[i] = (i + 1) as i64 * max_heights[i] as i64;
            } else {
                prefix[i] = prefix[*stack.back().unwrap()]
                    + (i - stack.back().unwrap()) as i64 * max_heights[i] as i64;
            }
            stack.push_back(i);
        }
        let mut res = 0;
        let mut suffix = vec![0; n];
        let mut stack = VecDeque::new();
        for i in (0..n).rev() {
            while !stack.is_empty() && max_heights[i] < max_heights[*stack.back().unwrap()] {
                stack.pop_back();
            }
            if stack.is_empty() {
                suffix[i] = (n - i) as i64 * max_heights[i] as i64;
            } else {
                suffix[i] = suffix[*stack.back().unwrap()]
                    + (stack.back().unwrap() - i) as i64 * max_heights[i] as i64;
            }
            stack.push_back(i);
            res = res.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }
        res
    }

    pub fn maximum_sum_of_heights_v1(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut prefix = vec![0; n];
        let mut stack = vec![0; n];
        let mut top = 0;
        for i in 0..n {
            while top > 0 && max_heights[i] < max_heights[stack[top - 1]] {
                top -= 1;
            }
            if top == 0 {
                prefix[i] = (i + 1) as i64 * max_heights[i] as i64;
            } else {
                prefix[i] =
                    prefix[stack[top - 1]] + (i - stack[top - 1]) as i64 * max_heights[i] as i64;
            }
            stack[top] = i;
            top += 1;
        }
        let mut res = 0;
        let mut suffix = vec![0; n];
        let mut stack = vec![0; n];
        let mut top = 0;
        for i in (0..n).rev() {
            while top > 0 && max_heights[i] < max_heights[stack[top - 1]] {
                top -= 1;
            }
            if top == 0 {
                suffix[i] = (n - i) as i64 * max_heights[i] as i64;
            } else {
                suffix[i] =
                    suffix[stack[top - 1]] + (stack[top - 1] - i) as i64 * max_heights[i] as i64;
            }
            stack[top] = i;
            top += 1;
            res = res.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_maximum_sum_of_heights() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }

    #[test]
    fn test_maximum_sum_of_heights_v1() {
        assert_eq!(Solution::maximum_sum_of_heights_v1(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(
            Solution::maximum_sum_of_heights_v1(vec![6, 5, 3, 9, 2, 7]),
            22
        );
        assert_eq!(
            Solution::maximum_sum_of_heights_v1(vec![3, 2, 5, 5, 2, 3]),
            18
        );
    }
}
