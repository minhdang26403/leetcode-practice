impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let numbers: HashSet<i32> = nums.into_iter().collect();
        let mut result = 0;

        for num in &numbers {
            if !numbers.contains(&(num - 1)) {
                let mut length = 1;
                while numbers.contains(&(num + length)) {
                    length += 1;
                }
                result = std::cmp::max(result, length);
            }
        }

        result
    }
}
