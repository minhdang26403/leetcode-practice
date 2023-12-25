impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut key_map = HashMap::<Vec<u8>, Vec<String>>::with_capacity(strs.len());

        for s in strs {
            let mut key = s.clone().into_bytes();
            key.sort();
            key_map.entry(key).or_default().push(s);
        }

        key_map.into_values().collect::<Vec<Vec<String>>>()
    }
}
