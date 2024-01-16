use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut low, mut high) = (0i32, (m * n) as i32 - 1);

        while low <= high {
            let mid = low + (high - low) / 2;
            let umid = mid as usize;
            let val = matrix[umid / n][umid % n];
            match val.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid - 1,
            }
        }

        false
    }
}
