use super::Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 != 0
            || tomato_slices < cheese_slices * 2
            || cheese_slices * 4 < tomato_slices
        {
            return vec![];
        }

        vec![
            tomato_slices / 2 - cheese_slices,
            cheese_slices * 2 - tomato_slices / 2,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_num_of_burgers() {
        assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
        assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
        assert_eq!(Solution::num_of_burgers(0, 0), vec![0, 0]);
        assert_eq!(Solution::num_of_burgers(2, 1), vec![0, 1]);
    }
}
