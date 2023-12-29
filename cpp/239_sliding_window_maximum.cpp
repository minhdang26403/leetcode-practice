#include <deque>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> maxSlidingWindow(vector<int>& nums, int k) {
    deque<int> max_indices;
    vector<int> result;
    result.reserve(nums.size() - k + 1);
    for (int end = 0; end < nums.size(); end++) {
      int start = end - k + 1;
      if (!max_indices.empty() && max_indices.front() < start) {
        max_indices.pop_front();
      }
      while (!max_indices.empty() && nums[max_indices.back()] < nums[end]) {
        max_indices.pop_back();
      }
      max_indices.push_back(end);
      if (start >= 0) {
        result.push_back(nums[max_indices.front()]);
      }
    }
    return result;
  }
};
