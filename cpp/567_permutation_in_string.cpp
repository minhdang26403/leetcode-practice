#include <array>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkInclusion(string s1, string s2) {
    int len1 = s1.size(), len2 = s2.size();
    if (len1 > len2) {
      return false;
    }

    array<int, 26> letter_freq{};
    for (const auto c : s1) {
      letter_freq[c - 'a']++;
    }

    int diff = len1;
    for (int end = 0; end < s2.size(); end++) {
      int start = end - len1;
      if (start >= 0) {
        char c = s2[start];
        // If a letter doesn't appear in s1 or appear in s2
        // more than in s1, its frequency is less than 0
        if (letter_freq[c - 'a'] >= 0) {
          diff++;
        }
        letter_freq[c - 'a']++;
      }

      char c = s2[end];
      // If this letter appears in s1, decrement diff
      if (letter_freq[c - 'a'] > 0) {
        diff--;
      }
      letter_freq[c - 'a']--;
      if (diff == 0) {
        return true;
      }
    }

    return false;
  }
};
