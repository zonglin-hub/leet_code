use super::Solution;

impl Solution {
    /// 该函数使用了 Fibonacci 数列的思想来计算爬楼梯的方案数。
    /// 在函数中，首先初始化了变量 a 和 b，它们都等于 1。
    /// 然后使用 for_each 方法循环 n 次，每次将 b 加上 a，然后将 a 赋值为 b-a。
    /// 最后，函数返回 a，表示爬楼梯的方案数。
    /// 该算法的时间复杂度为 $O(n)$，空间复杂度为 $O(1)$。
    ///
    /// # Fibonacci 数列是一组非常经典的数列，定义如下：
    ///
    /// - 第1项为 1
    /// - 第2项为 1
    /// - 第n项为第(n-1)项和第(n-2)项的和，即 $F_n = F_{n-1} + F_{n-2}$
    ///
    /// 这个数列的前几项是：1，1，2，3，5，8，13……以此类推。
    /// Fibonacci 数列中的每一项都可以通过前两项相加得到，因此这个数列被广泛应用于数学、计算机科学、物理学、生物学等领域。
    /// 在计算爬楼梯的方案数时，我们使用 Fibonacci 数列的性质来简化计算。
    pub fn climb_stairs_v3(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        (0..n).for_each(|_| {
            b += a;
            a = b - a;
        });
        a
    }

    /// 这是采用递归实现的爬楼梯方法：
    ///
    /// - 当 n=1 时，只有一种方法，即爬 1 级楼梯。
    /// - 当 n=2 时，有两种方法，即爬 1 级楼梯两次或者直接爬 2 级楼梯。
    /// - 当 n>2 时，可以选择爬1级楼梯或2级楼梯，由于可以选择爬1级楼梯再爬1级楼梯或者直接爬2级楼梯，因此到达第n个台阶的方法数等于到达第(n-1)个台阶的方法数加上到达第(n-2)个台阶的方法数。
    ///
    /// 在这个方法中，我们使用了递归的方式，将问题不断拆解为子问题，直到问题规模缩小到最小的子问题。虽然代码简单易懂，但这个方法的缺点是在n比较大时会出现重复计算，导致效率低下。
    pub fn climb_stairs_v2(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => Self::climb_stairs_v2(n - 1) + Self::climb_stairs_v2(n - 2),
        }
    }

    /// 这是采用动态规划实现的爬楼梯方法：
    ///
    /// - 我们可以用一个dp数组来存储到达每个台阶的方法数，其中 `dp[i]` 表示到达第i个台阶的方法数。
    /// - 当n=1时，只有一种方法，即爬1级楼梯，所以`dp[1]=1`。
    /// - 当n=2时，有两种方法，即爬1级楼梯两次或者直接爬2级楼梯，所以`dp[2]=2`。
    /// - 对于n>2的情况，每次可以选择爬1级或2级楼梯，到达第n个台阶的方法数等于到达第(n-1)个台阶的方法数加上到达第(n-2)个台阶的方法数，即`dp[n]=dp[n-1]+dp[n-2]`。
    /// - 最后返回dp数组的第n个元素即可。
    ///
    /// 这个方法不会出现递归计算的问题，而是利用已经计算过的结果来计算当前值，从而避免了重复计算的问题。
    ///
    /// # 动态规划
    ///
    /// 动态规划是一种常用的解决多阶段决策过程最优化问题的方法。
    /// 所谓多阶段决策过程是指决策过程可以分为很多个阶段，每个阶段都需要做出决策从而影响后续的阶段；而最优化问题的目标是进行一系列决策后使得总收益最大，总成本最小等等。
    /// 动态规划的基本思想是将一个大问题分解成许多小问题，并把小问题的解存储下来，下一步基于已经求解出来的小问题的解来解决更大的问题。
    /// 通常来说，动态规划问题可以分为三个部分：状态转移方程、边界情况和初始状态。其中，状态转移方程是最核心的部分，它描述了如何从已经求解出的子问题的解推出原问题的解。
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize]
        }

        dp[n as usize]
    }

    /// 滚动数组
    pub fn climb_stairs_v1(n: i32) -> i32 {
        let mut p;
        let mut q = 0;
        let mut r = 1;

        for _ in 1..=n {
            p = q;
            q = r;
            r = p + q;
        }
        r
    }

    /// 通项公式
    pub fn climb_stairs_v4(n: i32) -> i32 {
        let sqrt5 = 5f64.sqrt();
        let part1 = (5f64 + sqrt5) * (0.5f64 + sqrt5 / 2f64).powi(n);
        let part2 = (5f64 - sqrt5) * (0.5f64 - sqrt5 / 2f64).powi(n);
        ((part1 + part2) / 10f64).round() as i32
    }
}

impl Solution {
    /// 矩阵快速幂
    pub fn climb_stairs_v5(n: i32) -> i32 {
        fn multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
            let mut c = vec![vec![0; 2]; 2];

            for i in 0..2 {
                for j in 0..2 {
                    c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
                }
            }
            c
        }

        fn pow(a: Vec<Vec<i32>>, n: i32) -> Vec<Vec<i32>> {
            let mut ret = vec![vec![1, 0], vec![0, 1]];
            let mut a = a;
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ret = multiply(&ret, &a);
                }
                n >>= 1;
                a = multiply(&a, &a);
            }
            ret
        }

        let q = vec![vec![1, 1], vec![1, 0]];
        let res = pow(q, n);
        res[0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_climb_stairs_v1() {
        assert_eq!(Solution::climb_stairs_v1(2), 2);
        assert_eq!(Solution::climb_stairs_v1(3), 3);
    }

    #[test]
    fn test_climb_stairs_v2() {
        assert_eq!(Solution::climb_stairs_v2(2), 2);
        assert_eq!(Solution::climb_stairs_v2(3), 3);
    }

    #[test]
    fn test_climb_stairs_v3() {
        assert_eq!(Solution::climb_stairs_v3(2), 2);
        assert_eq!(Solution::climb_stairs_v3(3), 3);
    }

    #[test]
    fn test_climb_stairs_v4() {
        assert_eq!(Solution::climb_stairs_v4(2), 2);
        assert_eq!(Solution::climb_stairs_v4(3), 3);
    }

    #[test]
    fn test_climb_stairs_v5() {
        assert_eq!(Solution::climb_stairs_v5(2), 2);
        assert_eq!(Solution::climb_stairs_v5(3), 3);
    }
}
