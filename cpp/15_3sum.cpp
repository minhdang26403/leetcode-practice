#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> threeSum(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    vector<vector<int>> result;
    int n = nums.size();
    for (int i = 0; i < n - 2; i++) {
      if (nums[i] > 0) {
        break;
      }
      if (i > 0 && nums[i - 1] == nums[i]) {
        continue;
      }
      int target = -nums[i];
      int left = i + 1, right = n - 1;
      while (left < right) {
        int val = nums[left] + nums[right];
        if (val < target) {
          ++left;
        } else if (val > target) {
          --right;
        } else {
          result.push_back({nums[i], nums[left], nums[right]});
          ++left;
          --right;
          while (left < right && nums[left - 1] == nums[left]) {
            ++left;
          }
          while (left < right && nums[right] == nums[right + 1]) {
            --right;
          }
        }
      }
    }
    return result;
  }
};
