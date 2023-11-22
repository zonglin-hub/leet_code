use super::Solution;

impl Solution {
    /// 这个函数是一个用于合并两个有序数组的程序。函数的输入是两个已经排序的数组`intervals`和`intervals1`，输出是一个新的排序后的数组。
    ///
    /// 函数首先将两个输入数组进行排序，然后使用迭代器`intervals.iter().skip(1)`遍历第二个数组`intervals1`中的每个元素。对于每个元素，函数检查该元素的起始值是否大于当前合并数组的结束值。
    /// 如果是，则将当前合并数组添加到输出数组中，并更新起始值和结束值为当前元素的起始值和结束值。
    /// 如果当前元素的起始值小于或等于当前合并数组的结束值，则将当前元素的结束值与当前合并数组的结束值进行比较，并更新当前合并数组的结束值为两者中的较大值。
    ///
    /// 最后，函数将当前合并数组添加到输出数组中，并返回输出数组。
    pub fn merge_56(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut ans = vec![];
        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);

        intervals.iter().skip(1).for_each(|f| {
            if f[0] > end {
                ans.push(vec![start, end]);
                start = f[0];
            }
            end = end.max(f[1]);
        });
        ans.push(vec![start, end]);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_merge_56() {
        assert_eq!(
            Solution::merge_56(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );

        assert_eq!(
            Solution::merge_56(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}
