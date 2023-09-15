//! 我的日程安排表 I

use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct MyCalendar {
    bts: BTreeSet<(i32, i32)>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Self {
            bts: BTreeSet::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(v) = self.bts.range((start, 0)..).next() {
            if v.1 < end {
                return false;
            }
        }
        self.bts.insert((end - 1, start));
        true
    }
}
