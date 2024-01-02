#include <stack>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  bool isValid(string s) {
    unordered_map<char, char> paren = {{'(', ')'}, {'{', '}'}, {'[', ']'}};
    stack<char> stack;
    for (const auto &c : s) {
      if (paren.count(c) > 0) {
        stack.push(c);
      } else {
        if (stack.empty() || paren[stack.top()] != c) {
          return false;
        }
        stack.pop();
      }
    }
    return stack.empty();
  }
};
