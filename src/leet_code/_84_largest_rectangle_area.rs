use std::cmp;

use super::Solution;

impl Solution {
    /// 这个函数是用于计算一系列高度构成的矩形中的最大面积。函数的参数是一个整数数组 `heights`，它表示每个矩形的高度。
    ///
    /// 函数的主要逻辑如下：
    ///
    /// 1. 获取输入数组 `heights` 的长度，并保存在变量 `len` 中。
    /// 2. 初始化一个变量 `maxs` 为 0，用于存储最大矩形面积的值。
    /// 3. 外层循环从 0 到 `len - 1`，遍历数组中的每个元素。在这个循环中，`i` 表示当前矩形的起始位置。
    /// 4. 内层循环也从 `i` 到 `len - 1`，用于找到以当前位置为起始位置的矩形。在这个循环中，`j` 表示当前矩形的结束位置。
    /// 5. 对于每个结束位置 `j`，我们找到从起始位置 `i` 到结束位置 `j` 之间的最小高度，保存在变量 `mins` 中。通过不断更新 `mins` 的值，我们始终保持其为当前矩形中的最小高度。
    /// 6. 然后，我们计算以当前最小高度 `mins` 为高度的矩形的面积，即 `mins * (j - i + 1)`。我们将这个面积与 `maxs` 进行比较，更新 `maxs` 为较大的值。
    /// 7. 内层循环结束后，我们找到了以当前起始位置 `i` 为基础的矩形中的最大面积。
    /// 8. 外层循环继续，直到遍历完数组中的所有元素。
    /// 9. 最后，函数返回变量 `maxs` 的值，即最大矩形的面积。
    ///
    /// 该函数使用了双重循环来遍历所有可能的矩形，并找到其中的最大面积。时间复杂度为 O(n^2)，其中 n 是输入数组的长度。这种实现方式相对简单直观，但对于大规模的输入可能不够高效。可以进一步优化算法以降低时间复杂度。
    #[allow(clippy::needless_range_loop)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut maxs = 0;

        for i in 0..len {
            let mut mins = i32::MAX;
            for j in i..len {
                mins = cmp::min(mins, heights[j]);
                maxs = cmp::max(maxs, mins * (j - i + 1) as i32);
            }
        }
        maxs
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
