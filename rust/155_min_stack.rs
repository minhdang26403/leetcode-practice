struct MinStack {
    values: Vec<i32>,
    min_values: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            values: vec![],
            min_values: vec![],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.values.push(val);
        match self.min_values.last() {
            None => self.min_values.push(val),
            Some(&last_val) if val <= last_val => self.min_values.push(val),
            _ => (),
        }
    }
    
    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.min_values.pop();
        }
        self.values.pop();
    }
    
    fn top(&self) -> i32 {
        *self.values.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_values.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */