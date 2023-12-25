impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        !nums.into_iter().all(|x| seen.insert(x))
    }
}
