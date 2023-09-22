use super::Solution;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }

        let count = (money - children) / 7;
        let money_left = money - count * 7 - children;
        let children_left = children - count;
        let result =
            if children_left == 0 && money_left > 0 || children_left == 1 && money_left == 3 {
                count - 1
            } else {
                count
            };

        result.max(0)
    }
}
