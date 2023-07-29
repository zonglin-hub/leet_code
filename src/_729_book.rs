#![allow(unused)]
use std::collections::BTreeSet;

struct MyCalendar {
    bts: BTreeSet<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            bts: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(v) = self.bts.range((start, 0)..).next() {
            if v.1 < end {
                return false;
            }
        }
        self.bts.insert((end - 1, start));
        true
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_book() {
        use std::collections::BTreeSet;
        use std::ops::Bound::Included;

        let mut set = BTreeSet::new();
        set.insert(3);
        set.insert(5);
        set.insert(8);
        for &elem in set.range((Included(&4), Included(&8))) {
            println!("{elem}");
        }
        assert_eq!(Some(&5), set.range(4..).next());
    }
}
