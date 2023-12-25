impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table = std::collections::HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            let diff = target - num;
            let i = i as i32;
            if table.contains_key(&diff) {
                return vec![table[&diff], i];
            }
            table.insert(num, i);
        }

        unreachable!();
    }
}
