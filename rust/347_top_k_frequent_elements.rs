impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut num_freq = HashMap::new();
        let mut max_freq: usize = 0;
        nums.into_iter().for_each(|num| {
            let entry = num_freq.entry(num).or_default();
            *entry += 1;
            max_freq = std::cmp::max(max_freq, *entry);
        });

        let mut buckets: Vec<Vec<i32>> = vec![vec![]; max_freq + 1];
        for (num, freq) in num_freq {
            buckets[freq].push(num);
        }

        let mut result = vec![];
        for i in (0..max_freq + 1).rev() {
            for num in &buckets[i] {
                result.push(*num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}
