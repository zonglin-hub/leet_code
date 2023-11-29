use super::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ret = Vec::new();
        let mut tmp = Vec::new();
        let mut sum = 0;
        for w in words.iter() {
            let w_len = w.len() as i32;
            let tmp_len = tmp.len();
            if sum + w_len + tmp_len as i32 <= max_width {
                sum += w_len;
                tmp.push(w.clone());
            } else {
                if tmp_len > 1 {
                    let r = (max_width - sum) as usize % (tmp_len - 1);
                    let d = (max_width - sum) as usize / (tmp_len - 1);
                    tmp.iter_mut().take(r).for_each(|t| t.push(' '));
                    ret.push(tmp.join(format!("{:<1$}", ' ', d).as_str()));
                } else {
                    ret.push(format!("{:<1$}", tmp[0], max_width as usize));
                }

                sum = w_len;
                tmp.clear();
                tmp.push(w.clone());
            }
        }
        if !tmp.is_empty() {
            ret.push(format!("{:<1$}", tmp.join(" "), max_width as usize));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_full_justify() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string()
                ],
                16
            ),
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string()
            ]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string()
                ],
                16
            ),
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string()
            ]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                    "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string()
                ],
                20
            ),
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string()
            ]
        );
    }
}
