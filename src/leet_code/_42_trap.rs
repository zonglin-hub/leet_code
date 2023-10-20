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

impl Solution {
    pub fn trap_(height: Vec<i32>) -> i32 {
        let five_fives = height.iter().scan(0, |state, x| {
            // state: 0 x: 2
            // state: 2 x: 0
            // state: 2 x: 1
            // state: 2 x: 4
            // println!("state: {} x: {}", state, x);
            *state = *x.max(state);
            Some((*state, x))
        });
        // let mut n = five_fives.clone();
        // Some((2, 2))
        // Some((2, 0))
        // Some((2, 1))
        // Some((4, 4))
        // None
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        let binding = Vec::from_iter(five_fives);
        let nas = binding.iter().rev(/* 这里 rev 处理 [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1] 数据 */).scan(0, |state, x| {
            // state: 0 x: (4, 4) => 4.min(0).max(4) => state = 4 => state <= x.0 && state >= x.1
            // state: 4 x: (2, 1) => 2.min(4).max(1) => state = 2
            // state: 2 x: (2, 0) => 2.min(2).max(0) => state = 2
            // state: 2 x: (2, 2) => 2.min(2).max(2) => state = 2
            // println!("state: {} x: {:?}", state, x);
            // *state = x.0.min(*state).max(*x.1);
            *state = x.0;

            // 4
            // 2
            // 2
            // 2
            // println!("{}", state);
            Some(*state - x.1)
        });

        // let mut n = nas.clone();
        // Some(0)
        // Some(1)
        // Some(2)
        // Some(0)
        // None
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());
        // println!("{:?}", n.next());

        nas.sum()
    }
}
