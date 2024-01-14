use super::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut a = Vec::with_capacity(1 << 12);
        let mut b = Vec::with_capacity(1 << 12);
        a.push('1');
        for _ in 1..n {
            b.clear();
            let mut lst = '0';
            let mut cnt = 0;
            for &c in a.iter() {
                if lst != c {
                    if cnt > 0 {
                        b.extend(cnt.to_string().chars());
                        b.push(lst);
                    }
                    lst = c;
                    cnt = 1;
                } else {
                    cnt += 1
                }
            }
            b.extend(cnt.to_string().chars());
            b.push(lst);
            std::mem::swap(&mut a, &mut b)
        }
        a.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_owned());
    }
}
