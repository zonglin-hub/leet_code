pub struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        // 函数成立 必须 相等 
        if indices.len() != s.len() {
            return "".to_string();
        }

        let mut res = s.clone();
        let mut s = s.clone();

        unsafe {
            // 将res 和s 的 指针 指向同一个 内存区域
            let r = res.as_mut_vec();
            let sd = s.as_mut_vec();

            // 遍历 indices，将s 中的字符 替换 到res中
            for idx in 0..indices.len() {

                // vec![4, 5, 6, 7, 0, 2, 1, 3]
                let indx = indices[idx as usize] as usize;
                // c o d e l e e t
                // res[4] = sd[0] = c 
                // sd 索引 0 指向字节 c（原始指针s）再把 c 赋值给 res[4] 的值
                //（这个前提s, indices 为一一对应的）
                // 4 5 6 7 0 2 1 3
                // c o d e l e e t
                // l e e t c o d e
                // 0 1 2 3 4 5 6 7
                // 所以这个等式才成立
                r[indx] = sd[idx as usize];
            }
        }

        // 返回替换后的字符串
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_restore_string() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        // assert_eq!(
        //     Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
        //     "abc".to_string()
        // );
    }
}
