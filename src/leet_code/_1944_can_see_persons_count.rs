use super::Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; heights.len()];
        let mut stack = vec![];

        for (i, &val) in heights.iter().enumerate().rev() {
            while !stack.is_empty() && *stack.last().unwrap() < val {
                stack.pop();
                res[i] += 1;
            }

            if !stack.is_empty() {
                res[i] += 1;
            }

            stack.push(val);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_can_see_persons_count() {
        assert_eq!(
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        );
        assert_eq!(
            Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
            vec![4, 1, 1, 1, 0]
        );
    }
}
