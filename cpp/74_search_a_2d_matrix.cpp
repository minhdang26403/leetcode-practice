#include <vector>

using namespace std;

class Solution {
 public:
  bool searchMatrix(vector<vector<int>>& matrix, int target) {
    int m = matrix.size();
    int n = matrix[0].size();
    int low = 0, high = m * n - 1;
    while (low <= high) {
      int mid = low + (high - low) / 2;
      int val = matrix[mid / n][mid % n];
      if (val < target) {
        low = mid + 1;
      } else if (val > target) {
        high = mid - 1;
      } else {
        return true;
      }
    }
    return false;
  }
};
