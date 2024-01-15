#include <vector>
#include <stack>

using namespace std;

class Solution {
 public:
  int largestRectangleArea(vector<int>& heights) {
    heights.push_back(0);
    int area = 0;
    stack<int> st;
    for (int right = 0; right < heights.size(); right++) {
      while (!st.empty() && heights[st.top()] > heights[right]) {
        int height = heights[st.top()];
        st.pop();
        int left = (st.empty()) ? -1 : st.top();
        area = max(area, (right - left - 1) * height);
      }
      st.push(right);
    }

    return area;
  }
};
