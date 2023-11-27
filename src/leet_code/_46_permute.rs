use super::Solution;

impl Solution {
    /// 这个函数 `permute` 的目的是生成给定整数数组 `nums` 的所有排列。它使用深度优先搜索 (DFS) 的递归方法来实现。
    ///
    /// 函数的结构：
    ///
    /// 1. 定义了一个内部递归函数 `dfs`，该函数使用回溯的方法生成所有可能的排列。
    /// 2. 主函数 `permute` 创建一个空的 `ans` 数组来存储所有生成的排列，然后调用 `dfs` 函数。
    ///
    /// 内部递归函数 `dfs` 的工作原理：
    ///
    /// 1. 如果当前数字 `begin` 等于数组 `nums` 的长度，说明我们已经到达了数组的末尾，此时将 `nums` 转换为向量并添加到结果数组 `ans` 中。
    /// 2. 从 `begin` 开始遍历到数组 `nums` 的末尾。对于每个索引 `i`，交换索引 `i` 和 `begin` 处的元素，然后递归调用 `dfs`，将 `begin + 1` 作为新的开始索引。
    /// 3. 在递归调用返回后，交换索引 `i` 和 `begin` 处的元素，以便尝试其他可能的组合。
    ///
    /// 这个函数使用了一个优化，即交换而不是复制元素。在递归过程中，我们只需要考虑数组的不同部分，因此不需要复制整个数组。通过交换元素的位置，我们可以生成相同的排列而不需要额外的内存。
    ///
    /// 总之，这个函数使用深度优先搜索和递归回溯的方法来生成给定整数数组的所有排列。
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
            if begin == nums.len() {
                return ans.push(nums.to_vec());
            }

            for i in begin..nums.len() {
                nums.swap(i, begin);
                dfs(nums, ans, begin + 1);
                nums.swap(i, begin);
            }
        }
        let mut ans = vec![];
        dfs(&mut nums, &mut ans, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_jump() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2]
            ]
        );

        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
