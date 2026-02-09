use crate::leet_code::Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, &num)| nums[((i as i32 + num) % n as i32 + n as i32) as usize % n])
            .collect()
    }

    pub fn construct_transformed_array_v1(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let n_i32 = n as i32;

        nums.iter()
            .enumerate()
            .map(|(i, &num)| {
                let new_index = (i as i32 + num).rem_euclid(n_i32) as usize;
                nums[new_index]
            })
            .collect()
    }

    pub fn construct_transformed_array_v2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let n_i32 = n as i32;

        nums.iter()
            .enumerate()
            .map(|(i, &num)| {
                // 计算新的索引，处理负数情况
                let mut new_index: i32 = i as i32 + num;
                new_index = ((new_index % n_i32) + n_i32) % n_i32;

                nums[new_index as usize]
            })
            .collect()
    }

    pub fn construct_transformed_array_v3(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let result: Vec<i32> = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                // 1. 计算新索引（可能为负数）
                let raw_index = index as i32 + value;

                // 2. 标准化到 [0, n-1] 范围
                let normalized_index = raw_index.rem_euclid(n as i32);

                // 3. 获取对应元素
                nums[normalized_index as usize]
            })
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_construct_transformed_array() {
        assert_eq!(Solution::construct_transformed_array(vec![3, -2, 1, 1]), vec![1, 1, 1, 3]);
        assert_eq!(Solution::construct_transformed_array(vec![-1, 4, -1]), vec![-1, -1, 4]);
    }
}
