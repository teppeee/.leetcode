/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars:Vec<char> = s.chars().collect();
        for i in 0..=chars.len()-1 {
            for j in 0..=i {
                let len = chars.len() - i;
                let target = &chars[j..j+len];
                if check_palindromic(target){
                    return  target.iter().collect();
                }
    
            }
        }
    
        s.chars().nth(0).unwrap().to_string()
    }
}

pub fn check_palindromic(s:&[char]) -> bool {
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

