use super::Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut n = Vec::new();
        let mut a = Vec::new();

        for (i, v) in s.chars().enumerate() {
            if v == c {
                n.push(i as i32);
            }
        }

        let mut min;
        for i in 0..s.len() {
            min = 10000;
            for v in n.iter() {
                let m = i32::abs(v - i as i32);
                if min > m {
                    min = m;
                }
            }
            a.push(min);
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_owned(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(Solution::shortest_to_char("aaab".to_owned(), 'b'), vec![3, 2, 1, 0]);
    }
}
