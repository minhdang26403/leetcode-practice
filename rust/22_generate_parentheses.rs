impl Solution {
    fn generate_helper(result: &mut Vec<String>, paren: &mut String, open: i32, close: i32) {
        if open == 0 && close == 0 {
            result.push(paren.clone());
            return;
        }

        if open > 0 {
            paren.push('(');
            Solution::generate_helper(result, paren, open - 1, close);
            paren.pop();
        }
        if close > open {
            paren.push(')');
            Solution::generate_helper(result, paren, open, close - 1);
            paren.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut paren = "".to_string();
        Solution::generate_helper(&mut result, &mut paren, n, n);
        result
    }
}
