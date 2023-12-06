#![allow(unused)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

type NodePtr = Rc<RefCell<Node>>;

struct Node {
    key: i32,
    values: i32,
    prev: Option<NodePtr>,
    next: Option<NodePtr>,
}

impl Node {
    fn new(key: i32, values: i32) -> NodePtr {
        Rc::new(RefCell::new(Node {
            key,
            values,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy: NodePtr,
    key_to_node: HashMap<i32, NodePtr>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        LRUCache {
            capacity,
            dummy,
            key_to_node: HashMap::new(),
        }
    }

    fn remove(&mut self, x: NodePtr) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    fn push_front(&mut self, x: NodePtr) {
        let next = self.dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(self.dummy.clone());
        x.borrow_mut().next = next.clone();
        self.dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x);
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.key_to_node.get(&key) {
            Some(node) => {
                let node = node.clone();
                let value = node.borrow().values;
                self.remove(node.clone());
                self.push_front(node);
                value
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            let node = node.clone();
            node.borrow_mut().values = value;
            self.remove(node.clone());
            self.push_front(node);
            return;
        }

        let node = Node::new(key, value);
        self.key_to_node.insert(key, node.clone());
        self.push_front(node);

        if self.key_to_node.len() > self.capacity {
            let back_node = self.dummy.borrow().prev.clone().unwrap();
            self.key_to_node.remove(&back_node.borrow().key);
            self.remove(back_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);

        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
