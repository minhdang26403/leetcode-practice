#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  void countFrequency(unordered_map<int, int> &num_freq,
                      const vector<int> &nums) {
    for (const auto &num : nums) {
      num_freq[num]++;
    }
  }

  void fillBuckets(vector<vector<int>> &buckets,
                   const unordered_map<int, int> &num_freq) {
    for (const auto &[key, value] : num_freq) {
      buckets[value - 1].push_back(key);
    }
  }

  vector<int> topKFrequent(vector<int> &nums, int k) {
    unordered_map<int, int> num_freq;
    countFrequency(num_freq, nums);

    vector<vector<int>> buckets(nums.size());
    fillBuckets(buckets, num_freq);

    vector<int> result;
    for (int i = buckets.size() - 1; i > -1; --i) {
      for (const auto &num : buckets[i]) {
        result.push_back(num);
        if (result.size() == k) {
          return result;
        }
      }
    }
    return result;
  }
};
