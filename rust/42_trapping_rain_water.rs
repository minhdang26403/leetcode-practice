impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] < left_max {
                    water += left_max - height[left];
                } else {
                    left_max = height[left];
                }
                left += 1;
            } else {
                if height[right] < right_max {
                    water += right_max - height[right];
                } else {
                    right_max = height[right];
                }
                right -= 1;
            }
        }

        water
    }
}
