//! LCP 50. 宝石补给

use super::Solution;

impl Solution {
    pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        let mut gem = gem;
        // 遍历operations中的每一个op
        for op in operations {
            // 计算gem中op[0]位置的值减去op[1]位置的值
            let num = gem[op[0] as usize] / 2;
            // 将op[0]位置的值减去num
            gem[op[0] as usize] -= num;
            // 将op[1]位置的值加上num
            gem[op[1] as usize] += num;
        }
        // 返回gem中最大值减去最小值
        *gem.iter().max().expect("") - *gem.iter().min().expect("")
    }
}
