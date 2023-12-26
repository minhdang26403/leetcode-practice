#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<string>> groupAnagrams(vector<string> &strs) {
    unordered_map<string, vector<string>> map;
    for (const auto &s : strs) {
      string key = s;
      sort(key.begin(), key.end());
      map[key].push_back(s);
    }
    vector<vector<string>> result;
    for (auto &[key, words] : map) {
      result.push_back(std::move(words));
    }
    return result;
  }
};
