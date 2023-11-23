use super::Solution;

impl Solution {
    /// 这个函数的功能是计算一个包含重复元素的数组的所有子集。它使用深度优先搜索（DFS）来实现。下面是对这个函数的解释：
    ///
    /// 首先，函数定义了一个名为`dfs`的辅助函数，它接受三个参数：当前的索引`arg`、数组`nums`、存放所有子集的向量`ans`和存放当前子集的向量`path`。
    /// 这个函数的功能是从当前索引开始，递归地生成所有的子集，并将它们添加到`ans`中。如果当前索引大于数组的长度或者当前元素与前一个元素相同，那么它会跳过这个元素，继续递归。
    /// 否则，它会将当前元素添加到`path`中，并递归地生成下一个子集。最后，它会将当前子集添加到`ans`中，并将`path`还原为原始状态。
    /// 在主函数中，首先对数组`nums`进行排序，然后初始化存放所有子集的向量`ans`和存放当前子集的向量`path`。接下来，调用`dfs`函数，从索引 0 开始，递归地生成所有的子集，并将它们添加到`ans`中。最后，返回`ans`。
    ///
    /// 这个函数的时间复杂度为 O(2^n)，其中 n 是数组的长度。这是因为在最坏情况下，每个元素都有两种选择（包含或不包含），所以总共需要生成 2^n 个子集。空间复杂度为 O(n)，其中 n 是数组的长度。这是因为在最坏情况下，`ans`和`path`都需要存储所有的子集，所以总共需要 O(n) 的空间。
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(arg: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            ans.push(path.clone());
            for i in arg..nums.len() {
                if i > arg && nums[i] == nums[i - 1] {
                    continue;
                }
                path.push(nums[i]);
                dfs(i + 1, nums, ans, path);
                path.pop();
            }
        }

        nums.sort();
        let mut ans = Vec::<Vec<i32>>::new();
        dfs(0, &mut nums, &mut ans, &mut Vec::<i32>::new());
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_subsets_with_dup() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );

        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }
}
