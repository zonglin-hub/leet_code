#![allow(unused)]

use std::time::SystemTime;

fn sum_of_n(n: i64) -> i64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ns: [i64; 3] = [100000, 5000000, 1000000];
        let mut nanos = [0, 0, 0, 0, 0];
        for &num in ns.iter() {
            for i in 0..5 {
                let now = SystemTime::now();
                let _sum = sum_of_n(num);
                let duration = now.elapsed().unwrap();
                nanos[i] = duration.as_nanos() as i32;
            }

            let mut nanos_sum = 0;
            for &nano in nanos.iter() {
                nanos_sum += nano;
            }

            let time = nanos_sum / (nanos.len() as i32);
            println!("func used {time} ns");
        }
    }
}
