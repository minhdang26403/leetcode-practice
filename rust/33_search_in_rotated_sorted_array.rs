impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0i32, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }

            if nums[mid as usize] < nums[right as usize] {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }
        -1
    }
}
