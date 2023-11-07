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

    pub fn _full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        // 用于存储最终结果的向量
        let mut ret = Vec::new();
        // 临时存储当前行单词的向量
        let mut tmp = Vec::new();
        // 记录当前行字符的总数
        let mut sum = 0;

        // 遍历单词列表
        for w in words.iter() {
            // 当前单词的长度
            let w_len = w.len() as i32;
            // 当前行已存储的单词数量
            let tmp_len = tmp.len();

            // 判断添加当前单词后是否超宽
            if sum + w_len + tmp_len as i32 <= max_width {
                sum += w_len;
                tmp.push(w.clone());
            } else {
                // 处理超宽行的情况
                if tmp_len > 1 {
                    // 计算需要添加的空格数量
                    let r = (max_width - sum) as usize % (tmp_len - 1);
                    let d = (max_width - sum) as usize / (tmp_len - 1);
                    // 将部分空格添加到单词之间
                    tmp.iter_mut().take(r).for_each(|t| t.push(' '));
                    // 格式化当前行并添加到最终结果中
                    ret.push(tmp.join(format!("{:<1$}", ' ', d).as_str()));
                } else {
                    // 格式化只有一个单词的行并添加到最终结果中
                    ret.push(format!("{:<1$}", tmp[0], max_width as usize));
                }

                // 重置当前行的状态，准备处理下一行
                sum = w_len;
                tmp.clear();
                tmp.push(w.clone());
            }
        }

        // 处理最后一行（如果有未处理的单词）
        if !tmp.is_empty() {
            ret.push(format!("{:<1$}", tmp.join(" "), max_width as usize));
        }

        // 返回最终结果
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
