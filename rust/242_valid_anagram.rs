impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut letter_freq = [0; 26];
        s.bytes()
            .for_each(|c| letter_freq[(c - b'a') as usize] += 1);
        t.bytes()
            .for_each(|c| letter_freq[(c - b'a') as usize] -= 1);

        letter_freq.into_iter().all(|&x| x == 0)
    }
}
