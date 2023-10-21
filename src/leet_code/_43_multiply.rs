use super::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let to_vev = |x: String| {
            x.chars()
                .rev()
                .map(|x| (x as u8 - b'0') as i32)
                .collect::<Vec<_>>()
        };
        let mut mul = vec![0; num1.len() + num2.len()];
        let c1 = to_vev(num1);
        let c2 = to_vev(num2);
        for i in 0..c1.len() {
            for j in 0..c2.len() {
                mul[i + j] += c1[i] * c2[j];
            }
        }
        let mut add = 0;
        for item in &mut mul {
            let m = (*item + add) % 10;
            add = (*item + add) / 10;
            *item = m;
        }
        mul.iter()
            .rev()
            .enumerate()
            .skip_while(|(k, x)| x == &&0 && *k != mul.len() - 1)
            .map(|(_, x)| (*x as u8 + b'0') as char)
            .collect()
    }
}
