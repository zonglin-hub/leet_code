use super::Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            if s.starts_with('0') && s.len() > 1 {
                return false;
            }
            s.parse::<u8>().is_ok()
        }

        fn dfs(s: &str, addr: &mut Vec<String>, ans: &mut Vec<String>) {
            if s.len() > 12 - addr.len() * 3 || s.len() < 4 - addr.len() {
                return;
            }

            if addr.len() == 3 && is_valid(s) {
                addr.push(s[..].to_string());
                ans.push(addr.join(".").to_string());
                addr.pop();
                return;
            }

            for end in 1..s.len() {
                if !is_valid(&s[0..end]) {
                    break;
                }

                addr.push(s[..end].to_string());
                dfs(&s[end..], addr, ans);
                addr.pop();
            }
        }

        let mut ans = vec![];
        dfs(&s, &mut vec![], &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_restore_ip_addresses() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0".to_string()]
        );
        assert_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            vec![
                "1.0.10.23".to_string(),
                "1.0.102.3".to_string(),
                "10.1.0.23".to_string(),
                "10.10.2.3".to_string(),
                "101.0.2.3".to_string()
            ]
        );
    }
}
