use super::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut queue = Vec::new();
        path.split('/').for_each(|f| match f {
            "." | "" => (),
            ".." => {
                queue.pop();
            }
            _ => queue.push(f),
        });
        "/".to_string() + &queue.join("/")
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_simplify_path() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
    }

    #[test]
    fn test_empty_tuple() {
        let mut vec = vec![1, 2, 3];
        let _a = {
            vec.pop();
        };
        let a = vec.pop();
        assert_eq!(a, Some(2));
        assert_eq!(vec, [1]);
    }
}
