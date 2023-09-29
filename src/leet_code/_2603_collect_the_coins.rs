//! 收集树中金币

use std::collections::{HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();

        // 初始化图
        let mut d = edges
            .iter()
            .fold(vec![HashSet::<i32>::new(); n], |mut acc, v| {
                acc[v[0] as usize].insert(v[1]);
                acc[v[1] as usize].insert(v[0]);
                acc
            });

        // 初始化队列
        let mut q = (0..n)
            .filter(|i| d[*i].len() == 1 && coins[*i] == 0)
            .collect::<VecDeque<_>>();

        // 拓扑排序
        while let Some(x) = q.pop_front() {
            d[x].clone().into_iter().for_each(|y| {
                d[y as usize].remove(&(x as i32));

                if d[y as usize].len() == 1 && coins[y as usize] == 0 {
                    q.push_back(y as usize);
                }
            })
        }

        // 计算图的连通分量
        ((0..2)
            .fold(d, |mut acc, _| {
                acc.iter()
                    .enumerate()
                    .filter(|(_, d)| d.len() == 1)
                    .map(|(i, _)| i)
                    .collect::<VecDeque<_>>()
                    .into_iter()
                    .for_each(|x| {
                        acc[x].clone().iter().for_each(|y| {
                            acc[*y as usize].remove(&(x as i32));
                        });
                        acc[x].clear();
                    });

                acc
            })
            .iter()
            .filter(|d| !d.is_empty())
            .count()
            .max(1)
            .saturating_sub(1)
            * 2)
        .try_into()
        .expect("Panics: 类型转换")
    }
}
