use super::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        fn score(player: Vec<i32>) -> i32 {
            let mut res = 0;

            for i in 0..player.len() {
                if (i > 0 && player[i - 1] == 10) || (i > 1 && player[i - 2] == 10) {
                    res += 2 * player[i];
                } else {
                    res += player[i];
                }
            }
            res
        }

        match score(player1).cmp(&score(player2)) {
            Ordering::Equal => 0,
            Ordering::Less => 2,
            Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_winner() {
        assert_eq!(Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]), 1);
        assert_eq!(Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
        assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0);
    }
}
