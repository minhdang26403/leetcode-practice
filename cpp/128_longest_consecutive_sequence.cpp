#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestConsecutive(vector<int>& nums) {
    unordered_set<int> numbers(nums.begin(), nums.end());
    int result = 0;
    for (const auto& num : numbers) {
      if (numbers.count(num - 1) == 0) {
        int length = 1;
        while (numbers.count(num + length) > 0) {
          ++length;
        }
        result = max(result, length);
      }
    }
    return result;
  }
};
