use super::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        Vec::from_iter(height.iter().scan(0, |state, x| {
            *state = *x.max(state);
            Some((*state, x))
        }))
        .iter()
        .rev()
        .scan(0, |state, x| {
            *state = x.0.min(*state).max(*x.1);
            Some(*state - x.1)
        })
        .sum()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![2, 0, 1, 4]), 3);
    }
}
