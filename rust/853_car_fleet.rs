use std::cmp::Ordering;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrive_time: Vec<(i32, f32)> = position
            .into_iter()
            .zip(speed)
            .map(|(pos, speed)| (pos, (target - pos) as f32 / speed as f32))
            .collect();

        arrive_time.sort_unstable_by_key(|x| x.0);

        let mut result = 0;
        let mut max_time = 0f32;

        while let Some((_, time)) = arrive_time.pop() {
            if time > max_time {
                max_time = time;
                result += 1;
            }
        }

        result
    }
}
