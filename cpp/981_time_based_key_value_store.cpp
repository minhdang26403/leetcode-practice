#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class TimeMap {
 public:
  TimeMap() {}

  void set(string key, string value, int timestamp) {
    kv_store[key].emplace_back(value, timestamp);
  }

  string get(string key, int timestamp) {
    if (!kv_store.count(key)) {
      return "";
    }

    const auto& values = kv_store.at(key);
    if (values.front().second > timestamp) {
      return "";
    }

    int left = 0, right = values.size();
    while (left < right) {
      int mid = left + (right - left) / 2;
      if (values[mid].second <= timestamp) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    return values[right - 1].first;
  }

 private:
  unordered_map<string, vector<pair<string, int>>> kv_store;
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */