#include <array>
#include <string>

using namespace std;

class Solution {
 public:
  string minWindow(string s, string t) {
    if (s.size() < t.size()) {
      return "";
    }

    array<int, 58> letter_freq{};
    for (const auto c : t) {
      letter_freq[c - 'A']++;
    }

    int start = 0;
    int min_start = 0, min_len = s.size() + 1;
    int diff = t.size();

    for (int end = 0; end < s.size(); end++) {
      int& freq = letter_freq[s[end] - 'A'];
      if (freq > 0) {
        diff--;
      }
      freq--;

      while (diff == 0) {
        if (end - start + 1 < min_len) {
          min_len = end - start + 1;
          min_start = start;
        }
        int& freq = letter_freq[s[start] - 'A'];
        if (freq >= 0) {
          diff++;
        }
        freq++;
        start++;
      }
    }

    return (min_len < s.size() + 1) ? s.substr(min_start, min_len) : "";
  }
};