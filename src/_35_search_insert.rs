//! 搜索插入位置

use std::cmp::Ordering;

use crate::types::base_type::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // 遍历数组，找到target的位置
        for (i, &n) in nums.iter().enumerate() {
            // 如果target大于n，则返回i
            if n >= target {
                return i as i32;
            }
        }

        // if the target is greater than the last element, return the length of the array
        // 如果target小于等于最后一个元素，则返回数组长度
        nums.len() as i32
    }

    pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
        // 使用enumerate()方法遍历nums，返回一个包含i和n的对象
        nums.iter()
            // 使用find_map()方法查找target在nums中的位置，如果target大于nums中的元素，则返回i，否则返回None
            .enumerate()
            .find_map(|(i, n)| if n >= &target { Some(i as i32) } else { None })
            // 返回nums的长度，如果target大于nums中的元素，则返回i，否则返回nums的长度
            .unwrap_or(nums.len() as i32)
    }

    pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }

    pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            // 在二分查找中， mid 代表数组中间位置的下标，即 (left + right) / 2 。
            // 每一次循环都会通过比较 nums[mid] 和 target 的大小关系，来决定下一步搜索的方向。
            // 如果当前的 nums[mid] 小于 target，则说明目标值在右边，下一步应该搜索 mid 右侧的元素；
            // 反之，如果当前的 nums[mid] 大于 target，则说明目标值在左边，下一步应该搜索 mid 左侧的元素。
            // 当 nums[mid] == target 时，说明已经找到了目标值，直接返回下标 mid 即可。
            let mid = left + right / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => {
                    if mid == 0 {
                        return mid as i32;
                    }
                    right = mid - 1;
                }
            }
            // if nums[mid] == target {
            //     return mid as i32;
            // } else if nums[mid] < target {
            //     left = mid + 1;
            // } else {
            //     if mid == 0 {
            //         return mid as i32;
            //     }
            //     right = mid - 1;
            // }
        }
        left as i32
    }

    pub fn search_insert_4(nums: Vec<i32>, target: i32) -> i32 {
        // 使用fold函数，将nums中的每一个元素比target小的元素加1，
        // 如果比target大的元素，则加1后的结果就是target的位置
        nums.iter()
            .fold(0, |i, val| if val < &target { i + 1 } else { i })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert_1(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert_2(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert_3(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert_4(vec![1, 3, 5, 6], 7), 4);
    }
}
