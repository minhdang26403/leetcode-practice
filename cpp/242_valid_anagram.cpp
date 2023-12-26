#include <array>
#include <string>

using namespace std;

class Solution {
 public:
  constexpr int toIndex(char c) { return c - 'a'; }
  bool isAnagram(string s, string t) {
    if (s.size() != t.size()) {
      return false;
    }
    array<int, 26> letter_freq;
    letter_freq.fill(0);
    for (const auto &c : s) {
      letter_freq[toIndex(c)]++;
    }

    for (const auto &c : t) {
      if (letter_freq[toIndex(c)] == 0) {
        return false;
      }
      letter_freq[toIndex(c)]--;
    }
    return true;
  }
};
