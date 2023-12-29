impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 1 {
            return nums;
        }

        let mut result = Vec::with_capacity(nums.len() - k + 1);
        let mut max_indices = std::collections::VecDeque::with_capacity(k);
        max_indices.push_back(0 as usize);

        for end in 1..nums.len() {
            if max_indices.front().unwrap() + k - 1 < end {
                max_indices.pop_front();
            }
            while !max_indices.is_empty() && nums[*max_indices.back().unwrap()] < nums[end] {
                max_indices.pop_back();
            }
            max_indices.push_back(end);
            if end >= k - 1 {
                result.push(nums[*max_indices.front().unwrap()]);
            }
        }

        result
    }
}
