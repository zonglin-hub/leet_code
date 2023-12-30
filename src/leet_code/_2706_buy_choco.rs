use super::Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut fi = i32::MAX;
        let mut se = i32::MAX;

        for p in prices {
            if p < fi {
                se = fi;
                fi = p;
            } else if p < se {
                se = p;
            }
        }

        if money < fi + se {
            money
        } else {
            money - fi - se
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_buy_choco() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
