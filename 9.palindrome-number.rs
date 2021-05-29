/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let s : Vec<char>= x.to_string().chars().collect();
        let ret = check_palindromic(s);

        ret
    }
}

pub fn check_palindromic(s: Vec<char>) -> bool {
    let mut ret = true;
    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - i - 1] {
            ret = false;
            break;
        }
    }
    ret
}
// @lc code=end

