#include <utility>
#include <vector>

using namespace std;

class Solution {
 public:
  int carFleet(int target, vector<int>& position, vector<int>& speed) {
    int n = position.size();
    vector<pair<int, float>> arrive_time(n);
    for (int i = 0; i < n; ++i) {
      float time = (target - position[i]) / float(speed[i]);
      arrive_time[i] = {position[i], time};
    }

    sort(arrive_time.begin(), arrive_time.end());

    int result = 0;
    float max_time = 0;
    for (int i = n - 1; i > -1; --i) {
      float time = arrive_time[i].second;
      if (time > max_time) {
        max_time = time;
        result++;
      }
    }
    return result;
  }
};
