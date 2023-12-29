#include <array>
#include <string>

using namespace std;

class Solution {
 public:
  int characterReplacement(string s, int k) {
    array<int, 26> letter_freq{};
    int max_freq = 0;
    int start = 0, max_len = 0;
    for (int end = 0; end < s.size(); end++) {
      max_freq = max(max_freq, ++letter_freq[s[end] - 'A']);
      if (end - start + 1 - max_freq > k) {
        letter_freq[s[start] - 'A']--;
        start++;
      }
      max_len = max(max_len, end - start + 1);
    }

    return max_len;
  }
};