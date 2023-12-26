impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let letters = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        letters.clone().zip(letters.rev()).all(|(c1, c2)| c1 == c2)
    }
}
