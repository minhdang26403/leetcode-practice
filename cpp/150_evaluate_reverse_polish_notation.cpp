#include <stack>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 private:
  long Compute(long val1, long val2, const string &op) {
    switch (op[0]) {
      case '+':
        return val1 + val2;
      case '-':
        return val1 - val2;
      case '*':
        return val1 * val2;
      case '/':
        return val1 / val2;
    }
    return 0;
  }

 public:
  int evalRPN(vector<string> &tokens) {
    stack<long> stack;
    unordered_set<string> operators = {"+", "-", "*", "/"};
    for (const auto &token : tokens) {
      if (operators.count(token)) {
        long val2 = stack.top();
        stack.pop();
        long val1 = stack.top();
        stack.pop();
        long result = Compute(val1, val2, token);
        stack.push(result);
      } else {
        stack.push(stoi(token));
      }
    }
    return stack.top();
  }
};
