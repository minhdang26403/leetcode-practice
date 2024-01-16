#include <vector>

using namespace std;

class Solution {
 public:
  int computeTimeEat(const vector<int>& piles, int speed) {
    int count = 0;
    for (const auto& pile : piles) {
      count += pile / speed;
      if (pile % speed != 0) {
        ++count;
      }
    }
    return count;
  }
  int minEatingSpeed(vector<int>& piles, int h) {
    int low = 1;
    int high = *std::max_element(piles.begin(), piles.end()) + 1;
    while (low < high) {
      int eating_speed = low + (high - low) / 2;
      int time_eat = computeTimeEat(piles, eating_speed);
      if (time_eat > h) {
        low = eating_speed + 1;
      } else {
        high = eating_speed;
      }
    }
    return high;
  }
};
