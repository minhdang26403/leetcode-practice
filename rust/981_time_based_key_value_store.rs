use std::collections::HashMap;

struct Entry {
    value: String,
    timestamp: i32,
}

struct TimeMap {
    kv_store: HashMap<String, Vec<Entry>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self {
            kv_store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let values = self.kv_store.entry(key).or_default();
        values.push(Entry {value, timestamp});
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.kv_store.contains_key(&key) {
            return "".to_string();
        }

        let values = &self.kv_store[&key];
        if values.first().unwrap().timestamp > timestamp {
            return "".to_string();
        }

        let (mut left, mut right) = (0, values.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if values[mid].timestamp <= timestamp {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return values[right - 1].value.clone();
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */