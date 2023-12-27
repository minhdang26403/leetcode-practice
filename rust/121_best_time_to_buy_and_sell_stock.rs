impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut buy = prices[0];

        for price in prices {
            if price < buy {
                buy = price;
            } else {
                profit = std::cmp::max(profit, price - buy);
            }
        }

        profit
    }
}
