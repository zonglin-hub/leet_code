//! 将钱分给最多的儿童

use super::Solution;

impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money < children {
            return -1;
        }

        money -= children;
        let count = children.min(money / 7);
        money -= count * 7;
        children -= count;

        if children == 0 && money > 0 || children == 1 && money == 3 {
            count - 1
        } else {
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_dist_money() {
        assert_eq!(Solution::dist_money(20, 3), 1);
        assert_eq!(Solution::dist_money(16, 2), 2);
        assert_eq!(Solution::dist_money(23, 2), 1);
    }
}
