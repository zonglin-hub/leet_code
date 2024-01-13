use super::Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut arrays = vec![0; 256];
        s.bytes().for_each(|x| arrays[x as usize] += 1);
        let mut last = 0;
        let mut repeats = 0;
        (0..s.len()).fold(String::new(), |mut res, _| {
            for u in (b'a'..=b'z').rev() {
                if arrays[u as usize] == 0 {
                    continue;
                }
                if u == last {
                    if repeats >= repeat_limit {
                        continue;
                    }
                    repeats += 1;
                } else {
                    repeats = 1;
                    last = u;
                }
                arrays[u as usize] -= 1;
                res.push(u as char);
                break;
            }
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_repeat_limited_string() {
        assert_eq!(Solution::repeat_limited_string("cczazcc".to_owned(), 3), "zzcccac".to_owned());
        assert_eq!(Solution::repeat_limited_string("aababab".to_owned(), 2), "bbabaa".to_owned());
    }
}
