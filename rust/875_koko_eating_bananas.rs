impl Solution {
    fn compute_time_eat(piles: &Vec<i32>, speed: i32) -> i32 {
        let mut count = 0;
        for pile in piles {
            count += pile / speed;
            if pile % speed != 0 {
                count += 1;
            }
        }

        count
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut low, mut high) = (1i32, piles.iter().max().unwrap() + 1);
        while low < high {
            let eating_speed = low + (high - low) / 2;
            let time_eat = Solution::compute_time_eat(&piles, eating_speed);
            if time_eat > h {
                low = eating_speed + 1;
            } else {
                high = eating_speed;
            }
        }

        high
    }
}
