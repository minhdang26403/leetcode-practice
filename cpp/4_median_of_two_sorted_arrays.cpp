#include <vector>

using namespace std;

class Solution {
 public:
  double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
    int x_len = nums1.size(), y_len = nums2.size();
    if (x_len > y_len) {
      return findMedianSortedArrays(nums2, nums1);
    }
    int left = 0, right = x_len - 1;
    double result;
    while (true) {
      int partition_x = left + (right - left) / 2;
      int partition_y = (x_len + y_len + 1) / 2 - partition_x;
      int max_left_x = (partition_x == 0) ? INT_MIN : nums1[partition_x - 1];
      int min_right_x = (partition_x == x_len) ? INT_MAX : nums1[partition_x];
      int max_left_y = (partition_y == 0) ? INT_MIN : nums2[partition_y - 1];
      int min_right_y = (partition_y == y_len) ? INT_MAX : nums2[partition_y];
      if (max_left_x <= min_right_y && max_left_y <= min_right_x) {
        if ((x_len + y_len) % 2 == 0) {
          result =
              (max(max_left_x, max_left_y) + min(min_right_x, min_right_y)) /
              2.0;
        } else {
          result = max(max_left_x, max_left_y);
        }
        break;
      } else if (max_left_x > min_right_y) {
        right = partition_x - 1;
      } else {
        left = partition_x + 1;
      }
    }
    return result;
  }
};