impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let n = board.len();
        let mut row_set = vec![HashSet::new(); n];
        let mut col_set = vec![HashSet::new(); n];
        let mut subgrid_set = vec![HashSet::new(); n];

        for r in 0..n {
            for c in 0..n {
                let num = board[r][c];
                if num == '.' {
                    continue;
                }
                let subgrid_idx = (r / 3) * 3 + c / 3;
                if row_set[r].contains(&num)
                    || col_set[c].contains(&num)
                    || subgrid_set[subgrid_idx].contains(&num)
                {
                    return false;
                }
                row_set[r].insert(num);
                col_set[c].insert(num);
                subgrid_set[subgrid_idx].insert(num);
            }
        }

        true
    }
}
