impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let n = temperatures.len();
        let mut result = vec![0; n];

        for i in 0..n {
            while let Some(&day) = stack.last() {
                if temperatures[day] < temperatures[i] {
                    stack.pop();
                    result[day] = (i - day) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        result
    }
}
