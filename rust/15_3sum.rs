impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = Vec::new();
        let n = nums.len();

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let target = -nums[i];
            let (mut left, mut right) = (i + 1, n - 1);

            while left < right {
                let val = nums[left] + nums[right];
                if val < target {
                    left += 1;
                } else if (val > target) {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left - 1] == nums[left] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}
