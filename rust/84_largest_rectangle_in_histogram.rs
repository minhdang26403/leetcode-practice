use std::cmp;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut heights = heights;
        heights.push(0);
        let mut st = vec![];

        for right in 0..heights.len() {
            while let Some(&idx) = st.last() {
                if heights[idx] > heights[right] {
                    let height = heights[idx];
                    st.pop();
                    let left = if st.is_empty() {
                        -1
                    } else {
                        *st.last().unwrap() as i32
                    };
                    area = cmp::max(area, (right as i32 - left - 1) * height);
                } else {
                    break;
                }
            }
            st.push(right);
        }

        area
    }
}
