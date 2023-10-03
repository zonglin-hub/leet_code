//! 根据规则将箱子分类

use super::Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let a = (length | width | height) >= 10_000
            || (length * width * height) as i64 >= 1_000_000_000;

        let b = mass >= 100;

        if a && b {
            return "Both".to_string();
        }

        if a {
            return "Bulky".to_string();
        }

        if b {
            return "Heavy".to_string();
        }

        "Neither".to_string()
    }
}

impl Solution {
    pub fn categorize_box_(length: i32, width: i32, height: i32, mass: i32) -> String {
        let a = (length | width | height) >= 10_000
            || (length * width * height) as i64 >= 1_000_000_000;

        let b = mass >= 100;

        [
            (a && b, "Both"),
            (a, "Bulky"),
            (b, "Heavy"),
            (true, "Neither"),
        ]
        .iter()
        // 查找第一个满足条件的元素
        .find(|&(cond, _)| *cond)
        // 获取该元素的结果，并将其转换为字符串类型
        .map(|&(_, res)| res.to_string())
        // 如果找不到满足条件的元素，就返回一个默认的字符串结果。
        .unwrap_or_else(|| "Neither".to_string())
    }
}
