use super::Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        fn union(parent: &mut [usize], idx1: usize, idx2: usize) {
            parent[find(parent, idx1)] = find(parent, idx2);
        }

        fn find(parent: &mut [usize], mut idx: usize) -> usize {
            while parent[idx] != idx {
                parent[idx] = parent[parent[idx]];
                idx = parent[idx];
            }
            idx
        }

        let mut parent = vec![0; 128];
        (1..128).for_each(|f| parent[f] = f);
        let (equal, not_equal): (Vec<_>, Vec<_>) = equations
            .iter()
            .map(|f| f.bytes().collect())
            .partition(|x: &Vec<_>| x[1] == b'=');

        equal
            .iter()
            .for_each(|x| union(&mut parent, x[0] as usize, x[3] as usize));

        not_equal
            .iter()
            .all(|x| find(&mut parent, x[0] as usize) != find(&mut parent, x[3] as usize))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_equations_possible() {
        assert_eq!(
            Solution::equations_possible(vec!["b==a".to_string(), "a==b".to_string()]),
            true
        );
        assert_eq!(
            Solution::equations_possible(vec![
                "a==b".to_string(),
                "b==c".to_string(),
                "a==c".to_string()
            ]),
            true
        );

        assert_eq!(
            Solution::equations_possible(vec![
                "a==b".to_string(),
                "b!=c".to_string(),
                "c==a".to_string()
            ]),
            false
        );

        assert_eq!(
            Solution::equations_possible(vec![
                "c==c".to_string(),
                "b==d".to_string(),
                "x!=z".to_string()
            ]),
            true
        );
    }

    // #[test]
    // fn test_partition() {
    //     let a = [1, 2, 3];

    //     let (even, odd): (Vec<_>, Vec<_>) = a.into_iter().partition(|n| n % 2 == 0);

    //     assert_eq!(even, vec![2]);
    //     assert_eq!(odd, vec![1, 3]);
    // }
}
