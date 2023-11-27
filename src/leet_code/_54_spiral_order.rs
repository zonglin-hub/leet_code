use std::collections::VecDeque;

use super::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut deque = matrix
            .into_iter()
            .map(|f| f.into_iter().collect::<VecDeque<i32>>())
            .collect::<VecDeque<VecDeque<i32>>>();

        let mut res = vec![];
        let mut flag = 4;

        while !deque.is_empty() {
            match flag % 4 {
                0 => {
                    res.extend(deque.pop_front().unwrap().into_iter().collect::<Vec<i32>>());
                }
                1 => {
                    for i in deque.iter_mut() {
                        if let Some(value) = i.pop_back() {
                            res.push(value);
                        }
                    }
                }
                2 => {
                    let mut bottom = deque.pop_back().unwrap().into_iter().collect::<Vec<i32>>();
                    bottom.reverse();
                    res.extend_from_slice(&bottom);
                }
                3 => {
                    for i in (0..deque.len()).rev() {
                        let left = deque.get_mut(i).unwrap();
                        if let Some(value) = left.pop_front() {
                            res.push(value)
                        }
                    }
                }
                _ => {}
            }
            flag += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_spiral_order() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
