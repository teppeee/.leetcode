/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let (mut start, mut res, mut sign) = (false, 0i64, 1);
    
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    start = true;
                    res = res * 10 + (c as i64 - '0' as i64);
                    if res > std::i32::MAX as i64 {
                        break;
                    }
                },
                ' ' => {
                    if start {
                        break;
                    }
                },
                '+' => {
                    if start {
                        break;
                    }
                    sign = 1;
                    start = true;
                },
                '-' => {
                    if start {
                        break;
                    }
                    sign = -1;
                    start = true;
                }
                _ => break
            }
        }
    
        res *= sign;
        if res < std::i32::MIN as i64 {
            return std::i32::MIN;
        } else if res > std::i32::MAX as i64 {
            return std::i32::MAX;
        }
        res as i32
    }
}
// @lc code=end

