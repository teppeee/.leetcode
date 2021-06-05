/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        //prefix
        let mut chars: Vec<char> = strs[0].chars().collect();

        for str in strs {
            let targetchars: Vec<char> = str.chars().collect();
            let mut newchars: Vec<char> = vec![];
            for i in 0..chars.len() {
                if i < targetchars.len() && i < chars.len() {
                    if chars[i] == targetchars[i] {
                        newchars.push(chars[i])
                    } else {
                        break;
                    }
                }
            }
            chars = newchars;
        }

        chars.iter().collect()
    }
}
// @lc code=end
