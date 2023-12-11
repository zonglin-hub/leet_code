use super::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        Vec::from_iter(height.iter().scan(0, |state, x| {
            *state = *x.max(state);
            Some((*state, x))
        }))
        .iter()
        .rev()
        .scan(0, |state, x| {
            *state = x.0.min(*state).max(*x.1);
            Some(*state - x.1)
        })
        .sum()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![2, 0, 1, 4]), 3);
    }

    // #[test]
    // fn test_data() {
    //     let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    //     let mut n = height.iter().scan(0, |state, x| {
    //         // state: 0 x: 2
    //         // state: 2 x: 0
    //         // state: 2 x: 1
    //         // state: 2 x: 4
    //         // println!("state: {} x: {}", state, x);
    //         *state = *x.max(state);
    //         Some((*state, x))
    //     });
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     println!("{:?}", n.next());
    //     // Some((0, 0))            0 => 0 - 0 = 0
    //     // Some((1, 1))            1            0
    //     // Some((1, 0))            1            1
    //     // Some((2, 2))            2            0
    //     // Some((2, 1))            2            1
    //     // Some((2, 0))            2            2
    //     // Some((2, 1))            2            1
    //     // Some((3, 3))            3            0
    //     // Some((3, 2))            2            0
    //     // Some((3, 1))            2            1
    //     // Some((3, 2))            2            0
    //     // Some((3, 1)) => state = 1            0 这里 state 为水位, x.1 为 原始高度, 为什么使用rev 是因为第一遍 scan 迭代的 x.0 采取的是历史最高位
    //     // None
    // }

    // #[test]
    // fn test_scan() {
    //     let a = [1, 2, 3, 4];

    //     let mut iter = a.iter().scan(1, |state, &x| {
    //         // 每次迭代，我们将状态乘以元素 ...
    //         *state = *state * x;

    //         // ... 如果状态超过 6，则终止
    //         if *state > 6 {
    //             return None;
    //         }
    //         // ... else yield 状态的否定
    //         Some(-*state)
    //     });

    //     assert_eq!(iter.next(), Some(-1));
    //     assert_eq!(iter.next(), Some(-2));
    //     assert_eq!(iter.next(), Some(-6));
    //     assert_eq!(iter.next(), None);
    // }
}
