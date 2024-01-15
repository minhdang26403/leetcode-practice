impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in &tokens {
            if let Ok(val) = token.parse() {
                stack.push(val);
                continue;
            }
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                _ => unreachable!(),
            };
            stack.push(result);
        }

        *stack.last().unwrap()
    }
}
