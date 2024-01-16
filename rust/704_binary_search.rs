use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0i32, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            match target.cmp(&nums[mid as usize]) {
                Ordering::Equal => return mid,
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid - 1,
            }
        }

        -1
    }
}
