#![allow(unused)]

use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    frequency: i32,
    id: i32,
    key: i32,
    val: i32,
}

impl Node {
    fn new(id: i32, key: i32, val: i32) -> Self {
        Self {
            frequency: 1,
            id,
            key,
            val,
        }
    }
}

struct LFUCache {
    data: HashMap<i32, Node>,
    capacity: usize,
    primary_key: i32,
    order_set: BTreeSet<Node>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            data: Default::default(),
            capacity: capacity as usize,
            primary_key: 0,
            order_set: Default::default(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_key += 1;
                t.frequency += 1;
                t.id = self.primary_key;
                self.order_set.insert(*t);
                t.val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_key += 1;
                t.frequency += 1;
                t.id = self.primary_key;
                t.val = value;
                self.order_set.insert(*t);
            }
            None => {
                self.primary_key += 1;

                if self.capacity == self.data.len() {
                    let for_remove_node = *self.order_set.iter().next().unwrap();
                    self.data.remove(&for_remove_node.key);
                    self.order_set.remove(&for_remove_node);
                }

                let node = Node::new(self.primary_key, key, value);
                self.order_set.insert(node);
                self.data.insert(key, node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LFUCache;

    #[test]
    fn test_lfu_cache() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);

        assert_eq!(lfu.get(1), 1);

        lfu.put(3, 3);
        assert_eq!(lfu.get(2), -1);
        assert_eq!(lfu.get(3), 3);

        lfu.put(4, 4);

        assert_eq!(lfu.get(1), -1);
        assert_eq!(lfu.get(3), 3);
        assert_eq!(lfu.get(4), 4);
    }
}
