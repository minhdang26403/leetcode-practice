#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    int profit = 0;
    int buy = prices[0];
    for (const auto price : prices) {
      if (price < buy) {
        buy = price;
      } else {
        profit = max(profit, price - buy);
      }
    }

    return profit;
  }
};
