#![allow(unused)]

///
/// 这是一个使用计数排序的实现。计数排序（Counting Sort）是一种简单的排序算法，它首先统计每个元素出现的次数，然后根据统计结果将元素放入新的数组中。
pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    let mut occurences: Vec<usize> = vec![0; maxval + 1];
    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }
    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}

use std::ops::AddAssign;

/// 这是一个使用通用组合排序的实现。通用组合排序是一种通用的排序算法，它可以处理不同类型的整数切片，只需传入相应的类型即可。
pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy>(
    arr: &mut [T],
    maxval: usize,
) {
    let mut occurences: Vec<usize> = vec![0; maxval + 1];
    for &data in arr.iter() {
        occurences[data.into() as usize] += 1;
    }
    let mut i = 0;
    let mut data = T::from(0);
    for &number in occurences.iter() {
        for _ in 0..number {
            arr[i] = data;
            i += 1;
        }
        data += T::from(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting_sort() {
        let mut arr = [
            1, 2, 3, 2, 2, 2, 0, 1, 1, 3, 4, 5, 4, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ];
        counting_sort(&mut arr, 10);
    }
}
