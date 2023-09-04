#![allow(unused)]

///
/// 鸡尾酒排序（Cocktail Sort）是一种分布式排序算法，其原理是将一个数组分成多个子数组，然后对每个子数组进行排序，最后将所有子数组的数据合并。
///
pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    // 获取数组长度
    let len = arr.len();
    // 如果数组长度为0，则直接返回
    if len == 0 {
        return;
    }
    // 循环
    loop {
        // 如果数组中有一个元素大于等于另一个元素，则交换它们
        let mut swapped = false;
        for i in 0..(len - 1).clamp(0, len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        // 如果数组中没有元素大于等于另一个元素，则结束循环
        if !swapped {
            break;
        }
        // 如果数组中有一个元素大于另一个元素，则交换它们
        swapped = false;
        for i in (0..(len - 1).clamp(0, len)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        // 如果数组中没有元素大于另一个元素，则结束循环
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    #[test]
    fn one_element() {
        let mut arr = vec![1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
