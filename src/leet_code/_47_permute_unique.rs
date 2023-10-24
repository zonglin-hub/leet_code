use super::Solution;

impl Solution {
    /// 这个函数 `permute_unique` 的目的是生成一个给定整数数组 `nums` 的所有唯一排列。它使用深度优先搜索 (DFS) 的递归方法，结合了一些优化以处理重复数字。
    ///
    /// 函数的结构：
    ///
    /// 1. 定义了一个内部递归函数 `dfs`，该函数使用回溯的方法生成所有可能的排列。
    /// 2. 主函数 `permute_unique` 创建一个空的 `ans` 数组来存储所有生成的排列，然后调用 `dfs` 函数。
    ///
    /// 内部递归函数 `dfs` 的工作原理：
    ///
    /// 1. 如果当前组合 `com` 的长度等于输入数组 `nums` 的长度 `n`，则该组合是一个有效的排列，将其添加到结果数组 `ans` 中。
    /// 2. 遍历 `nums` 数组，忽略连续的重复元素。对于每个数字，将其添加到 `com` 中，并递归调用 `dfs`。这是一个回溯步骤，因为在下一层递归返回后，我们需要撤销这个动作（从 `com` 中删除这个数字）。
    /// 3. 在递归调用返回后，需要将刚刚添加的数字从 `com` 中移除，以便尝试其他可能的组合。
    ///
    /// 优化：
    ///
    /// 1. 在开始递归之前，先将 `nums` 排序，这样相同的数字会连续在一起。当我们遍历数组时，可以简单地忽略连续的相同数字，从而减少生成的排列数量。
    /// 2. 使用两个指针来生成新的数组 `new_nums`，一个指向当前数字之前的所有数字，另一个指向之后的数字。这确保了我们不会在相同的数字之间产生重复排列。
    ///
    /// 这个函数是一个经典的回溯算法示例，用于生成排列。它通过深度优先搜索和适当的剪枝来高效地处理重复数字，从而减少不必要的计算。
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(mut com: Vec<i32>, nums: &[i32], n: usize, ans: &mut Vec<Vec<i32>>) {
            if com.len() == n {
                return ans.push(com);
            }
            for (i, &num) in nums.iter().enumerate() {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                com.push(num);
                let new_nums = [&nums[..i], &nums[i + 1..]].concat();
                dfs(com.clone(), &new_nums, n, ans);
                com.pop();
            }
        }

        nums.sort_unstable();
        let mut ans = Vec::new();
        dfs(Vec::new(), &nums, nums.len(), &mut ans);
        ans
    }
}
