use std::cmp::max;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut letter_freq = [0; 26];
        let mut max_freq = 0;
        let (mut start, mut max_len) = (0, 0);
        let s_bytes = s.as_bytes();

        for (end, &c) in s_bytes.iter().enumerate() {
            let idx = (c - b'A') as usize;
            letter_freq[idx] += 1;
            max_freq = max(max_freq, letter_freq[idx]);
            if end - start + 1 - max_freq > k as usize {
                letter_freq[(s_bytes[start] - b'A') as usize] -= 1;
                start += 1;
            }
            max_len = max(max_len, end - start + 1);
        }

        max_len as i32
    }
}
