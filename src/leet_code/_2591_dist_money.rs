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
