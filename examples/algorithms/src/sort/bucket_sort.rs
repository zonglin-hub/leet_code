#![allow(unused)]

///
/// 这是一个使用桶排序的实现。桶排序是一种分布式排序算法，将一个数组分成多个桶，然后对每个桶进行排序，最后将所有桶的数据合并。
///
///
pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    // 创建一个10个空的Vec，用于存放每个位置的数组
    let mut bucket = vec![Vec::new(); 10];
    // 遍历数组，将每个元素放入桶中
    for i in arr {
        bucket[*i as usize / 10].push(*i);
    }
    // 创建一个空的Vec，用于存放排序后的数组
    let mut res = Vec::new();
    // 遍历桶，将每个桶中的元素放入空的Vec中
    for mut i in bucket {
        res.append(&mut i);
    }
    // 返回排序后的数组
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = bucket_sort(&arr);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
