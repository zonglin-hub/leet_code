impl super::Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack = vec![0];
        let iter = [0].iter().chain(&heights);
        let heights = iter.chain(&[0]).collect::<Vec<&i32>>();

        for (i, h) in heights.iter().enumerate().skip(1) {
            while heights[*stack.last().unwrap()] > h {
                ans =
                    ans.max(heights[stack.pop().unwrap()] * (i - stack.last().unwrap() - 1) as i32);
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
