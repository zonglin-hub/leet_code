use super::Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parent = vec![0; 128];

        (1..128).for_each(|f| parent[f] = f);

        let (equal, not_equal): (Vec<_>, Vec<_>) = equations
            .iter()
            .map(|f| f.bytes().collect())
            .partition(|x: &Vec<_>| x[1] == b'=');

        equal
            .iter()
            .for_each(|x| Self::union(&mut parent, x[0] as usize, x[3] as usize));

        not_equal.iter().all(|x| {
            Self::find(&mut parent, x[0] as usize) != Self::find(&mut parent, x[3] as usize)
        })
    }

    fn union(parent: &mut [usize], idx1: usize, idx2: usize) {
        let idx1 = Self::find(parent, idx1);
        let idx2 = Self::find(parent, idx2);
        parent[idx1] = idx2;
    }

    fn find(parent: &mut [usize], mut idx: usize) -> usize {
        while parent[idx] != idx {
            parent[idx] = parent[parent[idx]];
            idx = parent[idx];
        }
        idx
    }
}
