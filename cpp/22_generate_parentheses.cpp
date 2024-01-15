#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  void generateParen(vector<string> &result, string paren, int open,
                     int close) {
    if (open == 0 && close == 0) {
      result.push_back(std::move(paren));
      return;
    }
    if (open > 0) {
      generateParen(result, paren + "(", open - 1, close);
    }
    if (close > open) {
      generateParen(result, paren + ")", open, close - 1);
    }
  }
  vector<string> generateParenthesis(int n) {
    vector<string> result;
    generateParen(result, "", n, n);
    return result;
  }
};