#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  int lengthOfLongestSubstring(string s) {
    unordered_map<char, int> letter_idx;
    int start = 0, max_len = 0;
    for (int end = 0; end < s.size(); end++) {
      char c = s[end];
      if (letter_idx.count(c) && letter_idx.at(c) >= start) {
        start = letter_idx.at(c) + 1;
      }
      letter_idx[c] = end;
      max_len = max(max_len, end - start + 1);
    }

    return max_len;
  }
};