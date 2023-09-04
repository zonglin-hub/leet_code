#![allow(unused)]

///
/// 冒泡排序是一种简单的排序算法，通过重复地遍历列表，比较相邻的元素并在需要时交换它们，直到列表已排序。
///
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    // 如果数组长度小于等于1，则直接返回
    if arr.len() <= 1 {
        return;
    }

    // 定义一个变量size，表示数组长度
    let size = arr.len();
    // 循环遍历数组
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;
        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            // 如果当前循环发生元素交换，则交换位置
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
