//! 打家劫舍 IV

use super::Solution;

impl Solution {
    pub fn min_capability_2560(nums: Vec<i32>, k: i32) -> i32 {
        // 获取数组中最大值
        let mut max_num = *nums.iter().max().expect("存在最大值");
        // 获取数组中最小值
        let mut min_num = *nums.iter().min().expect("存在最小值");
        // 当最小值小于最大值时，循环
        while min_num <= max_num {
            // 计算中间值
            let middle = (min_num + max_num) / 2;
            // 定义计数器
            let mut count = 0;
            // 定义是否访问过,
            let mut visited = false;

            for &x in nums.iter() {
                if x <= middle && !visited {
                    count += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }

            // 如果计数器大于等于k，则最大值为中间值减1；否则，最小值为中间值加1
            if count >= k {
                max_num = middle - 1;
            } else {
                min_num = middle + 1;
            }
        }

        // 返回最小值
        min_num
    }
}
