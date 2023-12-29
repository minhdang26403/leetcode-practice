impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        if s.len() < t.len() {
            return "".to_string();
        }

        let mut letter_freq = [0; 58];
        for c in t {
            letter_freq[(c - b'A') as usize] += 1;
        }

        let mut start = 0;
        let (mut min_start, mut min_len) = (0, s.len() + 1);
        let mut diff = t.len();

        for (end, c) in s.iter().enumerate() {
            let freq = &mut letter_freq[(c - b'A') as usize];
            if *freq > 0 {
                diff -= 1;
            }
            *freq -= 1;

            while diff == 0 {
                if end - start + 1 < min_len {
                    min_len = end - start + 1;
                    min_start = start;
                }
                let freq = &mut letter_freq[(s[start] - b'A') as usize];
                if *freq >= 0 {
                    diff += 1;
                }
                *freq += 1;
                start += 1;
            }
        }

        if min_len < s.len() + 1 {
            std::str::from_utf8(&s[min_start..min_start + min_len])
                .unwrap()
                .to_string()
        } else {
            "".to_string()
        }
    }
}
