//! N 字形变换

use super::Solution;

impl Solution {
    pub fn convert_v1(s: String, num_rows: i32) -> String {
        (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .fold(
                vec![String::new(); num_rows as usize],
                |mut rows, (i, n)| {
                    rows[i as usize].push(n);
                    rows
                },
            )
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_convert_v1() {
        assert_eq!(
            Solution::convert_v1(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert_v1(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
        assert_eq!(
            Solution::convert_v1(String::from("A"), 1),
            String::from("A")
        );
    }
}
