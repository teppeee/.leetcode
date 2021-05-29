/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut nega = false;
        if x < 0 {
            nega = true;
        }
    
        let mut x = x.abs();
        let mut d = 0;
        let mut ans = 0;
        while  x != 0  {
            d = x % 10;
            let newans = ans * 10 + d;
            if(newans - d) /10 != ans{
                return 0;
            }
            ans = newans;
            x = x / 10;

        }
 
        if nega {
            ans = 0 - ans;
            ans
        } else {
            ans
        }
    }
}
// @lc code=end

