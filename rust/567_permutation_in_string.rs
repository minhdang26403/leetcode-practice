impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let (len1, len2) = (s1.len(), s2.len());
        if len1 > len2 {
            return false;
        }

        let mut letter_freq = [0; 26];
        for c in s1 {
            letter_freq[(c - b'a') as usize] += 1
        }

        let mut diff = len1;
        for end in 0..len2 {
            if end >= len1 {
                let c = s2[end - len1];
                if letter_freq[(c - b'a') as usize] >= 0 {
                    diff += 1;
                }
                letter_freq[(c - b'a') as usize] += 1;
            }

            let c = s2[end];
            if letter_freq[(c - b'a') as usize] > 0 {
                diff -= 1;
            }
            letter_freq[(c - b'a') as usize] -= 1;
            if diff == 0 {
                return true;
            }
        }

        false
    }
}
