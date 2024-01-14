//! 根据规则将箱子分类

use super::Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let mut i = 0;

        if (length | width | height) >= 10_000
            || length as i64 * width as i64 * height as i64 >= 1_000_000_000
        {
            i |= 1;
        }

        if mass >= 100 {
            i |= 2;
        }

        ["Neither", "Bulky", "Heavy", "Both"][i].to_owned()
    }

    pub fn categorize_box_v1(length: i32, width: i32, height: i32, mass: i32) -> String {
        let a = (length | width | height) >= 10_000
            || length as i64 * width as i64 * height as i64 >= 1_000_000_000;

        let b = mass >= 100;

        if a && b {
            return "Both".to_owned();
        }

        if a {
            return "Bulky".to_owned();
        }

        if b {
            return "Heavy".to_owned();
        }

        "Neither".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_categorize_box() {
        assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy".to_owned());
        assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither".to_owned());
        assert_eq!(Solution::categorize_box(2909, 3968, 3272, 727), "Both".to_owned());
    }

    #[test]
    fn test_categorize_box_v1() {
        assert_eq!(Solution::categorize_box_v1(1000, 35, 700, 300), "Heavy".to_owned());
        assert_eq!(Solution::categorize_box_v1(200, 50, 800, 50), "Neither".to_owned());
        assert_eq!(Solution::categorize_box_v1(2909, 3968, 3272, 727), "Both".to_owned());
    }
}
