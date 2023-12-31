impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product = 1;
        let n = nums.len();
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = product;
            product = product * nums[i];
        }

        product = 1;
        for i in (0..n).rev() {
            result[i] = result[i] * product;
            product = product * nums[i];
        }

        result
    }
}
