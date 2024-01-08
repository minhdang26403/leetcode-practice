#include <stack>
using namespace std;

class MinStack {
 public:
  MinStack() {}

  void push(int val) {
    values.push(val);
    if (min_values.empty() || val <= min_values.top()) {
      min_values.push(val);
    }
  }

  void pop() {
    int top_val = top();
    if (top_val == getMin()) {
      min_values.pop();
    }
    values.pop();
  }

  int top() { return values.top(); }

  int getMin() { return min_values.top(); }

 private:
  stack<int> values;
  stack<int> min_values;
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(val);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */