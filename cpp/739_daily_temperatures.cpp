#include <stack>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> dailyTemperatures(vector<int>& temperatures) {
    stack<int> stack;
    int n = temperatures.size();
    vector<int> result(n, 0);
    for (int i = 0; i < n; ++i) {
      while (!stack.empty() && temperatures[stack.top()] < temperatures[i]) {
        int day = stack.top();
        stack.pop();
        result[day] = i - day;
      }
      stack.push(i);
    }
    return result;
  }
};