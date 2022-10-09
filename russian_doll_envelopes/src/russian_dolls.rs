pub struct Solution {}
type Envelope = Vec<i32>;
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Envelope>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));
        let mut dp = vec![];
        for envelope in envelopes {
            let idx = dp.binary_search(&envelope[1]).unwrap_or_else(|x| x);
            if idx == dp.len() {
                dp.push(envelope[1]);
            } else {
                dp[idx] = envelope[1];
            }
        }
        dp.len() as i32
    }
}
