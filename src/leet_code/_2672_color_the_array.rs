//! 有相同颜色的相邻元素数目

use super::Solution;

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .fold(
                (vec![0; n as usize], Vec::new(), 0),
                |(mut color, mut res, mut cnt), query| {
                    let (i, x) = (query[0] as usize, query[1]);

                    if x == color[i] {
                        res.push(cnt);
                        return (color, res, cnt);
                    }

                    if color[i] != 0 && i as i32 > 0 && color[i - 1] == color[i] {
                        cnt -= 1;
                    }

                    if color[i] != 0 && i as i32 + 1 < n && color[i + 1] == color[i] {
                        cnt -= 1;
                    }

                    color[i] = x;

                    if i as i32 > 0 && color[i - 1] == color[i] {
                        cnt += 1;
                    }

                    if i as i32 + 1 < n && color[i + 1] == color[i] {
                        cnt += 1;
                    }

                    res.push(cnt);

                    (color, res, cnt)
                },
            )
            .1
    }
}
