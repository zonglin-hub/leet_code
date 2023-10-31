use super::Solution;

const X: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut chars = ('1'..='9').take(n as usize).collect::<Vec<_>>();
        find(&mut chars, k);
        String::from_iter(chars)
    }
}

fn find(chars: &mut [char], k: i32) {
    let n: usize = chars.len();
    if n <= 1 {
        return;
    }
    chars.sort();
    let idx = (k - 1) / X[n - 1];
    chars.swap(0, idx as usize);
    find(&mut chars[1..], k - idx * X[n - 1]);
}
