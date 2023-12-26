#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isValidSudoku(vector<vector<char>>& board) {
    int n = board.size();
    unordered_set<char> row_set[n];
    unordered_set<char> col_set[n];
    unordered_set<char> subgrid_set[n];
    for (int r = 0; r < n; ++r) {
      for (int c = 0; c < n; ++c) {
        char num = board[r][c];
        if (num == '.') {
          continue;
        }
        int subgrid_idx = (r / 3) * 3 + c / 3;
        if (row_set[r].count(num) || col_set[c].count(num) ||
            subgrid_set[subgrid_idx].count(num)) {
          return false;
        }
        row_set[r].insert(num);
        col_set[c].insert(num);
        subgrid_set[subgrid_idx].insert(num);
      }
    }
    return true;
  }
};
