use super::Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut ans = 1;
        let mut k = 1;

        (1..=time).for_each(|_| {
            ans += k;

            if ans == 1 || ans == n {
                k *= -1;
            }
        });
        ans
    }
}

impl Solution {
    pub fn pass_the_pillow_v1(n: i32, time: i32) -> i32 {
        let k = time / (n - 1);
        let md = time % (n - 1);
        if k & 1 == 1 {
            return n - md;
        }
        md + 1
    }
}
