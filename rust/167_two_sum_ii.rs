use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {    
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Less => left += 1,
                Greater => right -= 1,
                Equal => return vec![left as i32 + 1, right as i32 + 1],
            }
        }

        unreachable!("Test cases do not follow the constraints!")
    }
}
