use std::collections::BTreeMap;

use super::Solution;

impl Solution {
    /// 计算最小代价
    ///
    /// 参数:
    /// - nums: 原始数组
    /// - k: 需要选择的元素个数
    /// - dist: 相邻元素的最大距离约束
    ///
    /// 返回值:
    /// - 选择k个元素的最小代价（元素和）
    ///
    /// 算法思路:
    /// 1. 必须选择nums[0]
    /// 2. 使用滑动窗口维护中间k-2个最小元素
    /// 3. 遍历可能的最后一个元素位置，找到最小和
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;

        // 特殊情况处理
        if k == 1 {
            // 只需要选择第一个元素
            return nums[0] as i64;
        }

        if k == 2 {
            // 只需要选择第一个和最后一个元素，代价就是它们之和
            return (nums[0] + nums[n - 1]) as i64;
        }

        // 创建容器，维护最小的k-2个元素
        let mut min_elements_container = MinElementsContainer::new(k - 2);

        // 初始化：添加索引1到k-2的元素到容器中
        // 注意：这是选择中间k-2个元素的初始候选
        for i in 1..=(k - 2).min(n - 1) {
            min_elements_container.add(nums[i]);
        }

        // 初始答案：前k-1个元素的代价
        // nums[0]（必须）+ 中间k-2个最小元素的和 + 第k-1个元素
        let mut min_cost =
            if k - 1 < n { min_elements_container.sum() + nums[k - 1] as i64 } else { i64::MAX };

        // 滑动窗口：遍历可能的最后一个元素位置
        for last_element_idx in k..n {
            // 计算需要从窗口中移除的元素索引
            // 由于滑动窗口大小为dist+1，需要移除超出距离的元素
            let remove_idx = last_element_idx as i32 - dist as i32 - 1;

            // 安全地移除元素
            if remove_idx > 0 && (remove_idx as usize) < n {
                min_elements_container.remove(nums[remove_idx as usize]);
            }

            // 添加新元素到窗口（倒数第二个元素）
            if last_element_idx > 1 && last_element_idx - 1 < n {
                min_elements_container.add(nums[last_element_idx - 1]);
            }

            // 更新最小代价：当前中间k-2个最小元素的和 + 当前最后一个元素
            if last_element_idx < n {
                min_cost =
                    min_cost.min(min_elements_container.sum() + nums[last_element_idx] as i64);
            }
        }

        // 最终代价：最小中间元素和 + 必须的nums[0]
        min_cost + nums[0] as i64
    }
}

/// 维护最小的k个元素的数据结构
///
/// 使用两个BTreeMap分别存储：
/// - small_elements: 存储当前最小的k个元素
/// - large_elements: 存储剩余元素
///
/// 这样可以高效地：
/// 1. 添加元素
/// 2. 删除元素  
/// 3. 获取最小的k个元素的和
struct MinElementsContainer {
    k: usize,                           // 需要维护的最小元素数量
    small_elements: BTreeMap<i32, i32>, // 存储最小的k个元素（允许重复）
    large_elements: BTreeMap<i32, i32>, // 存储剩余元素
    sum_small: i64,                     // small_elements中所有元素的和
    small_count: usize,                 // small_elements中元素的数量
    large_count: usize,                 // large_elements中元素的数量
}

impl MinElementsContainer {
    /// 创建新的容器，维护最小的k个元素
    fn new(k: usize) -> Self {
        Self {
            k,
            small_elements: BTreeMap::new(),
            large_elements: BTreeMap::new(),
            sum_small: 0,
            small_count: 0,
            large_count: 0,
        }
    }

    /// 添加元素到容器中
    fn add(&mut self, value: i32) {
        // 如果large_elements不为空且value大于等于large_elements中的最小值
        // 或者small_elements为空，则将value添加到large_elements
        if !self.large_elements.is_empty() && value >= *self.large_elements.keys().next().unwrap() {
            Self::increment_count(&mut self.large_elements, value);
            self.large_count += 1;
        } else {
            // 否则添加到small_elements
            Self::increment_count(&mut self.small_elements, value);
            self.sum_small += value as i64;
            self.small_count += 1;
        }
        // 调整两个map，确保small_elements中恰好有k个元素
        self.balance();
    }

    /// 获取最小的k个元素的和
    fn sum(&self) -> i64 {
        self.sum_small
    }

    /// 从容器中移除元素
    fn remove(&mut self, value: i32) {
        // 尝试从small_elements中移除
        if Self::decrement_count(&mut self.small_elements, value) {
            self.sum_small -= value as i64;
            self.small_count -= 1;
        }
        // 如果在small_elements中没找到，尝试从large_elements中移除
        else if Self::decrement_count(&mut self.large_elements, value) {
            self.large_count -= 1;
        }
        // 调整两个map
        self.balance();
    }

    /// 平衡两个map，确保small_elements中恰好有k个元素
    fn balance(&mut self) {
        // 如果small_elements中的元素不足k个，从large_elements移动最小的元素过来
        while self.small_count < self.k && self.large_count > 0 {
            if let Some(&value) = self.large_elements.keys().next() {
                self.move_from_large_to_small(value);
            }
        }

        // 如果small_elements中的元素超过k个，将最大的元素移动到large_elements
        while self.small_count > self.k && self.small_count > 0 {
            if let Some(&value) = self.small_elements.keys().next_back() {
                self.move_from_small_to_large(value);
            }
        }
    }

    /// 将元素从large_elements移动到small_elements
    fn move_from_large_to_small(&mut self, value: i32) {
        if let Some(count) = self.large_elements.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.large_elements.remove(&value);
            }
            Self::increment_count(&mut self.small_elements, value);
            self.sum_small += value as i64;
            self.small_count += 1;
            self.large_count -= 1;
        }
    }

    /// 将元素从small_elements移动到large_elements
    fn move_from_small_to_large(&mut self, value: i32) {
        if let Some(count) = self.small_elements.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.small_elements.remove(&value);
            }
            Self::increment_count(&mut self.large_elements, value);
            self.sum_small -= value as i64;
            self.small_count -= 1;
            self.large_count += 1;
        }
    }

    /// 减少map中元素的计数，如果计数为0则移除
    fn decrement_count(map: &mut BTreeMap<i32, i32>, key: i32) -> bool {
        if let Some(count) = map.get_mut(&key) {
            *count -= 1;
            if *count == 0 {
                map.remove(&key);
            }
            true
        } else {
            false
        }
    }

    /// 增加map中元素的计数
    fn increment_count(map: &mut BTreeMap<i32, i32>, key: i32) {
        *map.entry(key).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_cost() {
        assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);

        // 边界测试
        assert_eq!(Solution::minimum_cost(vec![1], 1, 0), 1);
        assert_eq!(Solution::minimum_cost(vec![1, 2], 2, 1), 3);
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3], 2, 1), 4);
    }
}
