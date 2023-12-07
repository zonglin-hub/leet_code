#![allow(unused)]

use std::collections::LinkedList;

struct MyHashMap {
    base: Vec<LinkedList<(i32, i32)>>,
}

const BASE: i32 = 769;

impl MyHashMap {
    fn new() -> Self {
        Self {
            base: vec![LinkedList::new(); BASE as usize],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        for i in self.base[(key % BASE) as usize].iter_mut() {
            if i.0 == key {
                return i.1 = value;
            }
        }
        self.base[(key % BASE) as usize].push_back((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        for i in self.base[(key % BASE) as usize].iter() {
            if i.0 == key {
                return i.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let mut idx = None;
        for (i, item) in self.base[(key % BASE) as usize].iter().enumerate() {
            if item.0 == key {
                idx = Some(i);
                break;
            }
        }

        if let Some(idx) = idx {
            let mut split_list = self.base[(key % BASE) as usize].split_off(idx);
            split_list.pop_front();
            self.base[(key % BASE) as usize].append(&mut split_list);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashMap;

    #[test]
    fn test_my_hash_map() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1); // myHashMap 现在为 [[1,1]]
        obj.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]

        assert_eq!(obj.get(1), 1); // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
        assert_eq!(obj.get(3), -1); // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]

        obj.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
        assert_eq!(obj.get(2), 1); // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]

        obj.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
        assert_eq!(obj.get(2), -1); // 返回 -1（未找到），myHashMap 现在为 [[1,1]]
    }
}
