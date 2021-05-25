/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        use std::cmp;

        let mut len = 0;
        let mut j = 0;
        let mut map = HashMap::new();
        for (i, c) in s.char_indices() {
            if let Some(last) = map.insert(c, i + 1) {
                j = cmp::max(j, last)
            }
            len = cmp::max(len, i - j + 1);
        }
        len as i32
    }
}
// @lc code=end

