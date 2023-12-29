impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut letter_idx = std::collections::HashMap::new();
        let (mut start, mut max_len) = (0, 0);

        for (end, c) in s.as_bytes().iter().enumerate() {
            if letter_idx.contains_key(c) && letter_idx[c] >= start {
                start = letter_idx[c] + 1;
            }
            letter_idx.insert(c, end);
            max_len = std::cmp::max(max_len, end - start + 1);
        }

        max_len as i32
    }
}
